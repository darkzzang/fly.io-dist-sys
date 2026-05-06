use maelstrom_protocol::{Error as AppError, Message, MessageBody, Payload};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::time::Duration;

pub type ReplyState = Arc<(Mutex<Option<Payload>>, Condvar)>;
const TIMEOUT_DURATION: Duration = Duration::from_millis(1000);

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

pub struct Node {
    id: IdType,
    msg_id: AtomicUsize,
    counter: AtomicUsize,
    storage: Storage,
}

impl Node {
    pub fn new() -> Self {
        Self {
            id: IdType::None,
            msg_id: AtomicUsize::new(1),
            counter: AtomicUsize::new(1),
            storage: Storage::new(),
        }
    }

    pub fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn generate_msg_id(&self) -> usize {
        self.msg_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn set_messages(&mut self, msg: u64, src: Option<String>) -> bool {
        self.storage.set_messages(msg, src)
    }

    pub fn set_topology(&mut self, node_id: impl Into<String>, nodes: &[String]) {
        self.storage.set_topology(node_id, nodes);
    }

    pub fn messages(&self) -> Vec<u64> {
        self.storage.messages()
    }

    pub fn topology(&self) -> &HashMap<String, Vec<String>> {
        self.storage.topology()
    }

    pub fn neighbors(&self, k: &str) -> Vec<String> {
        let node_id = self.get_id();
        let topology = self.topology().get(&node_id).cloned().unwrap_or(vec![]);
        topology.into_iter().filter(|v| v != k).collect()
    }

    pub fn generate_id(&self) -> String {
        let count = self.counter.fetch_add(1, Ordering::Relaxed);
        format!("{}-{}", self.id, count)
    }

    pub fn update_ack_cursor(&mut self, target: &str, cursor: usize) {
        self.storage.update_ack_cursor(target, cursor);
    }

    pub fn ack_cursors(&self) -> HashMap<String, usize> {
        self.storage.ack_cursors()
    }

    pub fn update_sent_cursor(&mut self, target: &str, cursor: usize) {
        self.storage.update_sent_cursor(target, cursor);
    }

    pub fn sent_cursors(&self) -> HashMap<String, usize> {
        self.storage.sent_cursors()
    }

    pub fn sync_cursor(&mut self) {
        self.storage.sync_cursor();
    }

    pub fn get_source(&self, msg: u64) -> Option<String> {
        self.storage.get_source(msg)
    }

    pub fn sync_rpc_call(
        &self,
        dest: &str,
        payload: Payload,
        tx: &mpsc::Sender<Message>,
        pending_requests: &Arc<Mutex<HashMap<usize, ReplyState>>>,
    ) -> Result<Payload, AppError> {
        let msg_id = self.generate_msg_id();
        let reply_state: ReplyState = Arc::new((Mutex::new(None), Condvar::new()));

        {
            let reply_state_clone = Arc::clone(&reply_state);
            pending_requests
                .lock()
                .unwrap()
                .insert(msg_id, reply_state_clone);
        }

        let msg_body = MessageBody {
            msg_id: Some(msg_id),
            in_reply_to: None,
            payload,
        };
        let msg = Message {
            src: self.get_id(),
            dest: dest.to_string(),
            body: msg_body,
        };

        if let Err(e) = tx.send(msg) {
            pending_requests.lock().unwrap().remove(&msg_id);
            return Err(AppError::CustomError((1001, Some(e.to_string()))));
        }

        let (lock, cvar) = &*reply_state;
        let guard = lock.lock().unwrap();

        let mut result = cvar
            .wait_timeout_while(guard, TIMEOUT_DURATION, |payload| payload.is_none())
            .unwrap();

        if result.1.timed_out() {
            pending_requests.lock().unwrap().remove(&msg_id);
            Err(AppError::Timeout)
        } else {
            let recv_payload = result.0.take().unwrap();
            Ok(recv_payload)
        }
    }
}

pub struct Storage {
    messages: HashSet<u64>,
    ordered_messages: Vec<u64>,
    ack_cursors: HashMap<String, usize>,
    sent_cursors: HashMap<String, usize>,
    msg_src_list: HashMap<u64, String>,
    topology: HashMap<String, Vec<String>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            messages: HashSet::new(),
            ordered_messages: Vec::new(),
            ack_cursors: HashMap::new(),
            sent_cursors: HashMap::new(),
            msg_src_list: HashMap::new(),
            topology: HashMap::new(),
        }
    }

    pub fn set_messages(&mut self, msg: u64, source: Option<String>) -> bool {
        if self.messages.insert(msg) {
            self.ordered_messages.push(msg);
            if let Some(src) = source {
                self.msg_src_list.insert(msg, src);
            }
            true
        } else {
            false
        }
    }

    pub fn get_source(&self, msg: u64) -> Option<String> {
        self.msg_src_list.get(&msg).cloned()
    }

    pub fn set_topology(&mut self, node_id: impl Into<String>, nodes: &[String]) {
        self.topology
            .entry(node_id.into())
            .or_insert(nodes.to_owned());
    }

    pub fn messages(&self) -> Vec<u64> {
        self.ordered_messages.clone()
    }

    pub fn topology(&self) -> &HashMap<String, Vec<String>> {
        &self.topology
    }

    pub fn update_ack_cursor(&mut self, target: &str, cursor: usize) {
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

    pub fn ack_cursors(&self) -> HashMap<String, usize> {
        self.ack_cursors.clone()
    }

    pub fn update_sent_cursor(&mut self, target: &str, cursor: usize) {
        let key = target.to_string();
        self.sent_cursors.entry(key).insert_entry(cursor);
    }

    pub fn sent_cursors(&self) -> HashMap<String, usize> {
        self.sent_cursors.clone()
    }

    pub fn sync_cursor(&mut self) {
        let keys = self.sent_cursors.keys().cloned().collect::<Vec<String>>();

        keys.iter().for_each(|k| {
            let ack_cursor = self.ack_cursors.get(k).copied().unwrap_or(0);
            self.sent_cursors.insert(k.clone(), ack_cursor);
        });
    }
}
