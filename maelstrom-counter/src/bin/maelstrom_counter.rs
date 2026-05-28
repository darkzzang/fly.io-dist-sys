#[path = "../node.rs"]
mod node;

use maelstrom_protocol::{Message, MessageBody, Payload};
use node::{IdType, Node, ReplyState};
use std::collections::HashMap;
use std::io::{self, BufRead, Stdout, Write};
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};

const INTERVAL: u64 = 100;
// Theshold of latency challenge gave.(ms)
const THRESHOLD: u64 = 600;
// Latency of other node in challenge.(ms)
const LATENCY: u64 = 100;
// Intverval for async flush.(ms)
const ASYNC_INTERVAL: u64 = 1_000;

fn main() {
    let (tx, rx) = mpsc::channel::<Message>();
    let stdin = io::stdin();
    let mut msg_id_count = 0_usize;

    thread::spawn(move || {
        let mut stdout = io::stdout();

        while let Ok(msg) = rx.recv() {
            send(&mut stdout, msg);
        }
    });

    let node = Arc::new(RwLock::new(Node::new()));
    let pending_requests: Arc<Mutex<HashMap<usize, ReplyState>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let node_clone = Arc::clone(&node);
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let mut past_time = Instant::now();

        loop {
            thread::sleep(Duration::from_millis(INTERVAL));

            if time_limit_exceed(&mut past_time) {
                node_clone.write().unwrap().sync_cursor();
            }

            let (node_id, tot_messages, neighbors, sent_cursors, ack_cursors) = {
                let locked_node = node_clone.read().unwrap();
                let node_id = locked_node.get_id();
                let tot_messages = locked_node.messages();
                let neighbors = locked_node.neighbors(&node_id);
                let sent_cursors = locked_node.sent_cursors();
                let ack_cursors = locked_node.ack_cursors();

                if node_id == "uninitialized" || tot_messages.is_empty() {
                    continue;
                }
                (node_id, tot_messages, neighbors, sent_cursors, ack_cursors)
            };
            let mut new_cursors: Vec<(String, usize)> = Vec::new();

            for target in neighbors {
                let sent_cursor = *sent_cursors.get(&target).unwrap_or(&0);
                let ack_cursor = *ack_cursors.get(&target).unwrap_or(&0);

                if sent_cursor > ack_cursor {
                    continue;
                }

                let cursor = sent_cursor.min(tot_messages.len());
                let messages = tot_messages[cursor..].to_vec();

                if messages.is_empty() {
                    continue;
                }

                let mut filtered_msgs = Vec::new();

                {
                    let locked_node = node_clone.read().unwrap();
                    for msg in &messages {
                        if locked_node.get_source(*msg) != Some(target.clone()) {
                            filtered_msgs.push(*msg);
                        }
                    }
                }

                let new_cursor = cursor + messages.len();

                if filtered_msgs.is_empty() {
                    let mut locked_node = node_clone.write().unwrap();
                    locked_node.update_sent_cursor(&target, new_cursor);
                    locked_node.update_ack_cursor(&target, new_cursor);
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

                if tx_clone.send(msg).is_ok() {
                    new_cursors.push((target, new_cursor));
                }
            }

            new_cursors
                .iter()
                .for_each(|(t, c)| node_clone.write().unwrap().update_sent_cursor(t, *c));
        }
    });

    let node_clone_flush = Arc::clone(&node);
    let tx_clone_flush = tx.clone();
    let pending_requests_flush = Arc::clone(&pending_requests);

    thread::spawn(move || {
        let mut last_flushed_sum = 0;

        loop {
            thread::sleep(Duration::from_millis(ASYNC_INTERVAL));

            let (current_sum, node_id) = {
                let locked_node = node_clone_flush.read().unwrap();
                (locked_node.get_local_sum(), locked_node.get_id())
            };

            if node_id == "uninitialized" || current_sum == 0 || current_sum == last_flushed_sum {
                continue;
            }

            let partition_key = format!("counter_{}", node_id);

            let write_payload = Payload::Write {
                key: partition_key,
                value: current_sum as u64,
            };

            let write_result = node_clone_flush.read().unwrap().sync_rpc_call(
                "seq-kv",
                write_payload,
                &tx_clone_flush,
                &pending_requests_flush,
            );

            if let Ok(Payload::WriteOk) = write_result {
                last_flushed_sum = current_sum;
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

        if let Some(reply_to_id) = msg.body.in_reply_to {
            let reply_state_opt = {
                let mut pendings = pending_requests.lock().unwrap();
                pendings.remove(&reply_to_id)
            };

            if let Some(reply_state) = reply_state_opt {
                let (lock, cvar) = &*reply_state;
                {
                    let mut payload_guard = lock.lock().unwrap();
                    *payload_guard = Some(msg.body.payload);
                }
                cvar.notify_one();
            }
            continue;
        }

        let reply_payload = match msg.body.payload {
            Payload::Init { node_id, node_ids } => {
                let mut locked_node = node.write().unwrap();
                locked_node.set_id(IdType::Text(node_id.clone()));
                locked_node.set_all_nodes(node_ids.clone());

                let mut sorted_nodes = node_ids.clone();
                sorted_nodes.sort();

                if let Some(idx) = sorted_nodes.iter().position(|id| id == &node_id) {
                    let mut neighbors = Vec::new();

                    if idx == 0 {
                        for i in 1..sorted_nodes.len() {
                            neighbors.push(sorted_nodes[i].clone());
                        }
                    } else {
                        neighbors.push(sorted_nodes[0].clone());
                    }

                    locked_node.set_topology(&node_id, &neighbors);
                }
                Payload::InitOk
            }
            Payload::Echo { echo } => Payload::EchoOk { echo },
            Payload::Generate => {
                let unique_id = node.write().unwrap().generate_id();
                Payload::GenerateOk { id: unique_id }
            }
            Payload::Broadcast { message } => {
                node.write().unwrap().set_messages(message, None);
                Payload::BroadcastOk
            }
            Payload::Gossip { messages, cursor } => {
                {
                    let mut locked_node = node.write().unwrap();
                    for message in &messages {
                        locked_node.set_messages(*message, Some(msg.src.clone()));
                    }
                }

                if let Some(c) = cursor {
                    Payload::GossipOk { cursor: c }
                } else {
                    continue;
                }
            }
            Payload::GossipOk { cursor } => {
                node.write().unwrap().update_ack_cursor(&msg.src, cursor);
                continue;
            }
            Payload::Read { .. } => {
                let node_clone = Arc::clone(&node);
                let tx_clone = tx.clone();
                let client_src = msg.src.clone();
                let cliennt_msg_id = msg.body.msg_id;
                let pending_requests_clone = Arc::clone(&pending_requests);

                thread::spawn(move || {
                    let mut total_sum = 0_u64;
                    let (all_nodes, my_node_id) = {
                        let locked_node = node_clone.read().unwrap();
                        (locked_node.get_all_nodes(), locked_node.get_id())
                    };

                    for target_node in all_nodes {
                        if target_node == my_node_id {
                            total_sum += node_clone.read().unwrap().get_local_sum() as u64;
                            continue;
                        }

                        let read_payload = Payload::Read {
                            key: Some(format!("counter_{}", target_node)),
                        };
                        let read_result = node_clone.read().unwrap().sync_rpc_call(
                            "seq-kv",
                            read_payload,
                            &tx_clone,
                            &pending_requests_clone,
                        );
                        let current_value = match read_result {
                            Ok(Payload::ReadOk { value: Some(v), .. }) => v,
                            Ok(Payload::Error { code: 20, .. }) => 0,
                            _ => 0,
                        };

                        total_sum += current_value;
                    }

                    let reply_body = MessageBody {
                        msg_id: Some(node_clone.read().unwrap().generate_msg_id()),
                        in_reply_to: cliennt_msg_id,
                        payload: Payload::ReadOk {
                            messages: None,
                            value: Some(total_sum),
                        },
                    };
                    let reply_msg = set_reply_msg(&client_src, &my_node_id, &reply_body);

                    tx_clone.send(reply_msg).unwrap();
                });

                continue;
            }
            Payload::Topology { topology } => {
                for (node_id, nodes) in topology {
                    node.write().unwrap().set_topology(node_id, &nodes);
                }
                Payload::TopologyOk
            }
            Payload::Add { delta } => {
                node.read().unwrap().add_local(delta as usize);
                Payload::AddOk
            }
            _ => {
                continue;
            }
        };

        msg_id_count += 1;

        match reply_payload {
            Payload::Gossip { .. } => {
                continue;
            }
            _ => {
                let reply_body = MessageBody {
                    msg_id: Some(msg_id_count),
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

fn time_limit_exceed(time: &mut Instant) -> bool {
    let current_time = Instant::now();
    let time_limit = current_time
        .checked_sub(Duration::from_millis(THRESHOLD - LATENCY))
        .unwrap_or(Instant::now());

    if time_limit > *time {
        *time = current_time;
        true
    } else {
        false
    }
}
