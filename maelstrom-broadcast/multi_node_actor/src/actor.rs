use crate::node::{IdType, Node};
use maelstrom_protocol::{Message, MessageBody, Payload};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Stdin, Stdout};
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

const PERIOD: u64 = 100;

pub struct StdinActor {
    stdin: Stdin,
    tx: mpsc::UnboundedSender<Message>,
    node: Node,
}

impl StdinActor {
    pub fn new(tx: mpsc::UnboundedSender<Message>, node: Node) -> Self {
        Self {
            stdin: io::stdin(),
            tx,
            node,
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

                    let reply_payloads = match msg.body.payload {
                        Payload::Init { node_id, .. } => {
                            self.node.set_id(IdType::Text(node_id));
                            vec![Payload::InitOk]
                        }
                        Payload::Echo { echo } => vec![Payload::EchoOk { echo }],
                        Payload::Generate => {
                            let unique_id = self.node.generate_id();
                            vec![Payload::GenerateOk { id: unique_id }]
                        }
                        Payload::Broadcast { message } => {
                            self.node.set_messages(message);
                            vec![Payload::BroadcastOk]
                        }
                        Payload::Gossip { messages } => {
                            for message in messages {
                                self.node.set_messages(message);
                            }
                            vec![]
                        }
                        Payload::Read => vec![Payload::ReadOk {
                            messages: self.node.messages(),
                        }],
                        Payload::Topology { topology } => {
                            for (node_id, nodes) in topology {
                                self.node.set_topology(node_id, &nodes);
                            }
                            vec![Payload::TopologyOk]
                        }
                        _ => {
                            continue;
                        }
                    };

                    for reply_payload in reply_payloads {
                        let reply_body = match reply_payload {
                            Payload::Generate { .. } => MessageBody {
                                msg_id: Some(self.node.get_seq()),
                                in_reply_to: msg.body.msg_id,
                                payload: reply_payload,
                            },
                            _ => MessageBody {
                                msg_id: Some(self.node.next_msg_id()),
                                in_reply_to: msg.body.msg_id,
                                payload: reply_payload,
                            },
                        };

                        let msg = Message {
                            src: msg.dest.clone(),
                            dest: msg.src.clone(),
                            body: reply_body.clone(),
                        };

                        self.tx.send(msg).unwrap()
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read line: {}", e);
                    break;
                }
            }
        }
    }

    pub async fn run_gossip_timer(node: Node, tx: mpsc::UnboundedSender<Message>) {
        let mut ticker = interval(Duration::from_millis(PERIOD));

        loop {
            ticker.tick().await;

            let node_id = node.get_id();
            let known_msgs = node.messages();

            if known_msgs.is_empty() || node.is_uninitialized() {
                continue;
            }

            for target in node.neighbors(&node_id) {
                let gossip_msg = Message {
                    src: node_id.clone(),
                    dest: target.clone(),
                    body: MessageBody {
                        msg_id: None,
                        in_reply_to: None,
                        payload: Payload::Gossip {
                            messages: known_msgs.clone(),
                        },
                    },
                };

                tx.send(gossip_msg).unwrap();
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
