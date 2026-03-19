mod node;
use maelstrom_protocol::{Message, MessageBody, Payload};
use node::{IdType, Node};
use std::io::{self, BufRead, Stdout, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut msg_id_counter = 0;
    let mut node = Node::new();
    let mut buffer: Vec<Message> = Vec::new();

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

        let reply_payloads = match msg.body.payload {
            Payload::Init { node_id, .. } => {
                node.set_id(IdType::Text(node_id));
                vec![Payload::InitOk]
            }
            Payload::Echo { echo } => vec![Payload::EchoOk { echo }],
            Payload::Generate => {
                let unique_id = node.generate_id();
                vec![Payload::GenerateOk { id: unique_id }]
            }
            Payload::Broadcast { message } => {
                if node.set_messages(message) {
                    vec![
                        Payload::Gossip {
                            messages: vec![message],
                        },
                        Payload::BroadcastOk,
                    ]
                } else {
                    vec![Payload::BroadcastOk]
                }
            }
            Payload::Gossip { messages } => {
                let mut new_msg = Vec::new();

                for message in &messages {
                    if node.set_messages(*message) {
                        new_msg.push(*message);
                    }
                }

                if new_msg.is_empty() {
                    vec![]
                } else {
                    vec![Payload::Gossip { messages: new_msg }]
                }
            }
            Payload::Read => vec![Payload::ReadOk {
                messages: node.messages(),
            }],
            Payload::Topology { topology } => {
                for (node_id, nodes) in topology {
                    node.set_topology(node_id, &nodes);
                }
                vec![Payload::TopologyOk]
            }
            _ => {
                continue;
            }
        };

        msg_id_counter += 1;

        for reply_payload in reply_payloads {
            let reply_body = match reply_payload {
                Payload::Gossip { .. } => MessageBody {
                    msg_id: None,
                    in_reply_to: None,
                    payload: reply_payload,
                },
                _ => MessageBody {
                    msg_id: Some(msg_id_counter),
                    in_reply_to: msg.body.msg_id,
                    payload: reply_payload,
                },
            };

            set_reply_msgs(&mut buffer, &node, &msg.src, &msg.dest, &reply_body);

            if !buffer.is_empty() {
                send(&mut stdout, &mut buffer);
            }
        }
    }
}

fn set_reply_msgs(
    buffer: &mut Vec<Message>,
    node: &Node,
    src: &str,
    dest: &str,
    reply_body: &MessageBody,
) {
    let new_src = node.get_id();
    match reply_body.payload {
        Payload::Gossip { .. } => {
            for target in node.neighbors(&src) {
                buffer.push(Message {
                    src: new_src.to_string(),
                    dest: target,
                    body: reply_body.clone(),
                })
            }
        }
        _ => buffer.push(Message {
            src: dest.to_string(),
            dest: src.to_string(),
            body: reply_body.clone(),
        }),
    }
}

fn send(stdout: &mut Stdout, buffer: &mut Vec<Message>) {
    for reply_msg in buffer.drain(..) {
        let mut reply_json = match serde_json::to_string(&reply_msg) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Serialization error: {}", e);
                continue;
            }
        };
        reply_json.push('\n');

        if let Err(e) = stdout.write_all(reply_json.as_bytes()) {
            eprintln!("Failed to write to STDOUT: {}", e);
        }
        if let Err(e) = stdout.flush() {
            eprintln!("Failed to flush STDOUT: {}", e);
        }
    }
}
