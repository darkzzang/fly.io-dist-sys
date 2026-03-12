mod node;
use maelstrom_protocol::{Message, MessageBody, Payload};
use node::{IdType, Node};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut msg_id_counter = 0;
    let mut node = Node::new();

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
                node.set_id(IdType::Text(node_id));
                Payload::InitOk
            }
            Payload::Echo { echo } => Payload::EchoOk { echo },
            Payload::Generate => {
                let unique_id = node.generate_id();
                Payload::GenerateOk { id: unique_id }
            }
            _ => {
                continue;
            }
        };

        msg_id_counter += 1;

        let reply_body = MessageBody {
            msg_id: Some(msg_id_counter),
            in_reply_to: msg.body.msg_id,
            payload: reply_payload,
        };
        let reply_msg = Message {
            src: msg.dest.clone(),
            dest: msg.src.clone(),
            body: reply_body,
        };
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
