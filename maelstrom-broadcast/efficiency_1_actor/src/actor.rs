use maelstrom_protocol::{Message, MessageBody, Payload};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Stdin, Stdout};
use tokio::sync::mpsc;

pub struct StdinActor {
    stdin: Stdin,
    tx: mpsc::UnboundedSender<Event>,
}

impl StdinActor {
    pub fn new(tx: mpsc::UnboundedSender<Event>) -> Self {
        Self {
            stdin: io::stdin(),
            tx,
        }
    }

    pub async fn run_stdin_reader(&mut self) {
        let mut reader = BufReader::new(&mut self.stdin);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let msg = match serde_json::from_str::<Message>(&line) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Deserialization error: {} - Raw line: {}", e, line);
                            continue;
                        }
                    };

                    self.tx.send(Event::IncomingMessage(msg)).unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to read line: {}", e);
                    break;
                }
            }
        }
    }
}

pub struct StdoutActor {
    stdout: Stdout,
    rx: mpsc::UnboundedReceiver<Message>,
}

impl StdoutActor {
    pub fn new(rx: mpsc::UnboundedReceiver<Message>) -> Self {
        Self {
            stdout: io::stdout(),
            rx,
        }
    }

    pub async fn run_stdout(mut self) {
        while let Some(reply_msg) = self.rx.recv().await {
            let mut reply_json = match serde_json::to_string(&reply_msg) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Serialization error: {}", e);
                    continue;
                }
            };

            reply_json.push('\n');

            if let Err(e) = self.stdout.write_all(reply_json.as_bytes()).await {
                eprintln!("Failed to write to STDOUT: {}", e);
            }

            if let Err(e) = self.stdout.flush().await {
                eprintln!("Failed to flush STDOUT: {}", e);
            }
        }
    }
}

pub struct NodeActor {
    id: IdType,
    msg_seq: usize,
    messages: HashSet<u64>,
    ordered_messages: Vec<u64>,
    source_map: HashMap<u64, String>,
    topology: HashMap<String, Vec<String>>,
    sent_cursors: HashMap<String, usize>,
    ack_cursors: HashMap<String, usize>,
    rx: mpsc::UnboundedReceiver<Event>,
    tx: mpsc::UnboundedSender<Message>,
}

impl NodeActor {
    pub fn new(rx: mpsc::UnboundedReceiver<Event>, tx: mpsc::UnboundedSender<Message>) -> Self {
        Self {
            id: IdType::None,
            msg_seq: 1,
            messages: HashSet::new(),
            ordered_messages: Vec::new(),
            source_map: HashMap::new(),
            topology: HashMap::new(),
            sent_cursors: HashMap::new(),
            ack_cursors: HashMap::new(),
            rx,
            tx,
        }
    }

    pub async fn run(mut self) {
        while let Some(event) = self.rx.recv().await {
            match event {
                Event::IncomingMessage(msg) => self.handle_message(msg),
                Event::GossipTick => self.handle_gossip_tick(),
                Event::AntiEntropyTick => self.handle_anti_entropy_tick(),
            }
        }
    }

    pub fn is_uninitialized(&self) -> bool {
        matches!(self.id, IdType::None)
    }

    fn set_topology(&mut self, node_id: impl Into<String>, nodes: &[String]) {
        self.topology
            .entry(node_id.into())
            .or_insert(nodes.to_owned());
    }

    fn set_messages(&mut self, msg: u64, source: Option<String>) -> bool {
        if self.messages.insert(msg) {
            self.ordered_messages.push(msg);
            if let Some(src) = source {
                self.source_map.insert(msg, src);
            }
            true
        } else {
            false
        }
    }

    fn messages(&self) -> Vec<u64> {
        self.ordered_messages.clone()
    }

    fn neighbors(&self, k: &str) -> Vec<String> {
        let node_id = self.id.to_string();
        let topology = self.topology.get(&node_id).cloned().unwrap_or(vec![]);
        topology.into_iter().filter(|v| v != k).collect()
    }

    fn update_ack_cursor(&mut self, target: &str, cursor: usize) {
        let ack_cursor = self
            .ack_cursors
            .get(target)
            .copied()
            .unwrap_or(0)
            .max(cursor);

        self.ack_cursors
            .entry(target.to_string())
            .insert_entry(ack_cursor);
    }

    fn generate_id(&self) -> String {
        let last_seq = self.source_map.keys().max().unwrap_or(&1);
        format!("{}-{}", self.id, last_seq)
    }

    fn update_sent_cursor(&mut self, target: &str, cursor: usize) {
        let key = target.to_string();
        self.sent_cursors.entry(key).insert_entry(cursor);
    }

    fn get_source(&self, msg: u64) -> Option<String> {
        self.source_map.get(&msg).cloned()
    }

    fn sync_cursor(&mut self) {
        let keys = self.sent_cursors.keys().cloned().collect::<Vec<String>>();

        keys.iter().for_each(|k| {
            let ack_cursor = self.ack_cursors.get(k).copied().unwrap_or(0);
            self.sent_cursors.insert(k.clone(), ack_cursor);
        });
    }

    fn handle_message(&mut self, msg: Message) {
        if let Some(body) = self.handle_message_body(&msg) {
            let reply_msg = Message {
                src: msg.dest,
                dest: msg.src,
                body,
            };
            self.tx.send(reply_msg).unwrap();
        }
    }

    fn handle_gossip_tick(&mut self) {
        if !self.messages.is_empty() || !self.is_uninitialized() {
            let mut new_cursors = Vec::new();
            let node_id = self.id.to_string();

            for target in self.neighbors(&node_id) {
                let sent_cursor = self.sent_cursors.get(&target).unwrap_or(&0);
                let ack_cursor = self.ack_cursors.get(&target).unwrap_or(&0);

                if sent_cursor > ack_cursor {
                    continue;
                }

                let cursor = *sent_cursor.min(&self.messages.len());
                let messages = self.messages()[cursor..].to_vec();

                if messages.is_empty() {
                    continue;
                }

                let mut filtered_msgs = Vec::new();

                {
                    for msg in &messages {
                        if self.get_source(*msg) != Some(target.clone()) {
                            filtered_msgs.push(*msg);
                        }
                    }
                }

                let new_cursor = cursor + messages.len();

                if filtered_msgs.is_empty() {
                    self.update_sent_cursor(&target, new_cursor);
                    self.update_ack_cursor(&target, new_cursor);
                    continue;
                }

                let msg_body = MessageBody {
                    msg_id: None,
                    in_reply_to: None,
                    payload: Payload::Gossip {
                        messages: filtered_msgs,
                        cursor: Some(new_cursor),
                    },
                };
                let msg = Message {
                    src: node_id.clone(),
                    dest: target.clone(),
                    body: msg_body,
                };

                if self.tx.send(msg).is_ok() {
                    new_cursors.push((target, new_cursor));
                }
            }

            new_cursors
                .iter()
                .for_each(|(t, c)| self.update_sent_cursor(t, *c));
        }
    }

    fn handle_anti_entropy_tick(&mut self) {
        self.sync_cursor();
    }

    fn handle_message_body(&mut self, msg: &Message) -> Option<MessageBody> {
        if let Some(payload) = self.handle_payload(msg) {
            self.msg_seq += 1;

            Some(MessageBody {
                msg_id: Some(self.msg_seq),
                in_reply_to: msg.body.msg_id,
                payload,
            })
        } else {
            None
        }
    }

    fn handle_payload(&mut self, msg: &Message) -> Option<Payload> {
        match &msg.body.payload {
            Payload::Init { node_id, node_ids } => {
                self.id = IdType::Text(node_id.clone());

                let mut sorted_nodes = node_ids.clone();
                sorted_nodes.sort();

                // Construct Star topology
                if let Some(idx) = sorted_nodes.iter().position(|id| id == node_id) {
                    let length = (self.topology.len() * 2).max(1);
                    let mut neighbors = Vec::with_capacity(length);

                    if idx == 0 {
                        for (i, _) in sorted_nodes.iter().enumerate().skip(1) {
                            neighbors.push(sorted_nodes[i].clone());
                        }
                    } else {
                        neighbors.push(sorted_nodes[0].clone());
                    }

                    neighbors.shrink_to_fit();
                    self.set_topology(node_id, &neighbors);
                }
                Some(Payload::InitOk)
            }
            Payload::Echo { echo } => Some(Payload::EchoOk {
                echo: echo.to_string(),
            }),
            Payload::Generate => {
                let unique_id = self.generate_id();
                Some(Payload::GenerateOk { id: unique_id })
            }
            Payload::Broadcast { message } => {
                self.set_messages(*message, None);
                Some(Payload::BroadcastOk)
            }
            Payload::Gossip { messages, cursor } => {
                {
                    for message in messages {
                        self.set_messages(*message, Some(msg.src.clone()));
                    }
                }

                cursor.as_ref().map(|c| Payload::GossipOk { cursor: *c })
            }
            Payload::GossipOk { cursor } => {
                self.update_ack_cursor(&msg.src, *cursor);
                None
            }
            Payload::Read => Some(Payload::ReadOk {
                messages: self.messages(),
            }),
            Payload::Topology { topology } => {
                for (node_id, nodes) in topology {
                    self.set_topology(node_id, nodes);
                }
                Some(Payload::TopologyOk)
            }
            _ => None,
        }
    }
}

pub enum Event {
    IncomingMessage(Message),
    GossipTick,
    AntiEntropyTick,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum IdType {
    Integer(u64),
    Float(f64),
    Boolean(bool),
    Text(String),
    IntegerArray(Vec<u64>),
    FloatArray(Vec<f64>),
    TextArray(Vec<String>),
    BooleanArray(Vec<bool>),
    MixedArray(Vec<IdType>),
    None,
}

impl std::fmt::Display for IdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Text(s) => write!(f, "{s}"),
            Self::IntegerArray(ia) => {
                let v = ia.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                write!(f, "[{}]", v.join(", "))
            }
            Self::FloatArray(fa) => {
                let v = fa.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                write!(f, "[{}]", v.join(", "))
            }
            Self::TextArray(ta) => {
                let v = ta.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                write!(f, "[{}]", v.join(", "))
            }
            Self::BooleanArray(ba) => {
                let v = ba.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                write!(f, "[{}]", v.join(", "))
            }
            Self::MixedArray(ma) => {
                let v = ma.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                write!(f, "[{}]", v.join(", "))
            }
            Self::None => write!(f, "uninitialized"),
        }
    }
}
