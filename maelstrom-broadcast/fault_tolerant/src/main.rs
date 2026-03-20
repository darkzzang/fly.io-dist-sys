mod node;
use maelstrom_protocol::{Message, MessageBody, Payload};
use node::{IdType, Node};
use std::io::{self, BufRead, Stdout, Write};
use std::sync::{Arc, RwLock, mpsc};
use std::thread;
use std::time::Duration;

const INTERVAL: u64 = 100;

fn main() {
    let (tx, rx) = mpsc::channel::<Message>();
    let stdin = io::stdin();

    thread::spawn(move || {
        let mut stdout = io::stdout();

        while let Ok(msg) = rx.recv() {
            send(&mut stdout, msg);
        }
    });

    let node = Arc::new(RwLock::new(Node::new()));
    let node_clone = Arc::clone(&node);
    let tx_clone = tx.clone();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(INTERVAL));

            let (node_id, messages, neighbors) = {
                let locked_node = node_clone.read().unwrap();
                let node_id = locked_node.get_id();
                let messages = locked_node.messages();
                let neighbors = locked_node.neighbors(&node_id);

                if node_id == "uninitialized" || messages.is_empty() {
                    continue;
                }
                (node_id, messages, neighbors)
            };

            for target in neighbors {
                let msg_body = MessageBody {
                    msg_id: None,
                    in_reply_to: None,
                    payload: Payload::Gossip {
                        messages: messages.clone(),
                    },
                };
                let msg = Message {
                    src: node_id.clone(),
                    dest: target,
                    body: msg_body,
                };

                tx_clone.send(msg).unwrap();
            }
        }
    });

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to read from STDIN: {}", e);
                continue;
            }
        };
        let msg = match serde_json::from_str::<Message>(&line) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Deserialization error: {} - Raw line: {}", e, line);
                continue;
            }
        };

        let reply_payload = match msg.body.payload {
            Payload::Init { node_id, .. } => {
                node.write().unwrap().set_id(IdType::Text(node_id));
                Payload::InitOk
            }
            Payload::Echo { echo } => Payload::EchoOk { echo },
            Payload::Generate => {
                let unique_id = node.write().unwrap().generate_id();
                Payload::GenerateOk { id: unique_id }
            }
            Payload::Broadcast { message } => {
                node.write().unwrap().set_messages(message);
                Payload::BroadcastOk
            }
            Payload::Gossip { messages } => {
                let mut locked_node = node.write().unwrap();
                for message in &messages {
                    locked_node.set_messages(*message);
                }
                Payload::Gossip { messages }
            }
            Payload::Read => Payload::ReadOk {
                messages: node.write().unwrap().messages(),
            },
            Payload::Topology { topology } => {
                for (node_id, nodes) in topology {
                    node.write().unwrap().set_topology(node_id, &nodes);
                }
                Payload::TopologyOk
            }
            _ => {
                continue;
            }
        };

        match reply_payload {
            Payload::Gossip { .. } => {
                continue;
            }
            _ => {
                let reply_body = MessageBody {
                    msg_id: Some(1), // FIXME
                    in_reply_to: msg.body.msg_id,
                    payload: reply_payload,
                };
                let msg = set_reply_msg(&msg.src, &msg.dest, &reply_body);

                tx.send(msg).unwrap();
            }
        }
    }
}

fn set_reply_msg(src: &str, dest: &str, reply_body: &MessageBody) -> Message {
    Message {
        src: dest.to_string(),
        dest: src.to_string(),
        body: reply_body.clone(),
    }
}

fn send(stdout: &mut Stdout, reply_msg: Message) {
    let mut reply_json = serde_json::to_string(&reply_msg).unwrap();
    reply_json.push('\n');

    if let Err(e) = stdout.write_all(reply_json.as_bytes()) {
        eprintln!("Failed to write to STDOUT: {}", e);
    }
    if let Err(e) = stdout.flush() {
        eprintln!("Failed to flush STDOUT: {}", e);
    }
}
