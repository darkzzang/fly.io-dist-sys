use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

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
    counter: AtomicUsize,
    storage: Storage,
}

impl Node {
    pub fn new() -> Self {
        Self {
            id: IdType::None,
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

    pub fn set_messages(&mut self, msg: u64) -> bool {
        self.storage.set_messages(msg)
    }

    pub fn set_topology(&mut self, node_id: impl Into<String>, nodes: &Vec<String>) {
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
}

pub struct Storage {
    messages: HashSet<u64>,
    topology: HashMap<String, Vec<String>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            messages: HashSet::new(),
            topology: HashMap::new(),
        }
    }

    pub fn set_messages(&mut self, msg: u64) -> bool {
        self.messages.insert(msg)
    }

    pub fn set_topology(&mut self, node_id: impl Into<String>, nodes: &Vec<String>) {
        self.topology
            .entry(node_id.into())
            .and_modify(|v| {
                v.extend_from_slice(nodes);
            })
            .or_insert(nodes.clone());
    }

    pub fn messages(&self) -> Vec<u64> {
        self.messages.iter().cloned().collect()
    }

    pub fn topology(&self) -> &HashMap<String, Vec<String>> {
        &self.topology
    }
}
