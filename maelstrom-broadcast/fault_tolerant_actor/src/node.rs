use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

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

#[derive(Clone, Debug)]
pub(crate) struct Node {
    counter: Arc<AtomicUsize>,
    state: SharedNode,
}

impl Node {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(1)),
            state: Arc::new(RwLock::new(NodeState::new())),
        }
    }

    pub fn set_id(&self, id: IdType) {
        self.state.write().unwrap().id = id;
    }

    pub fn get_id(&self) -> String {
        self.state.read().unwrap().id.to_string()
    }

    pub fn get_seq(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn set_messages(&self, msg: u64) -> bool {
        self.state.write().unwrap().set_messages(msg)
    }

    pub fn set_topology(&self, node_id: impl Into<String>, nodes: &[String]) {
        self.state.write().unwrap().set_topology(node_id, nodes);
    }

    pub fn messages(&self) -> Vec<u64> {
        self.state.read().unwrap().messages()
    }

    pub fn topology(&self) -> HashMap<String, Vec<String>> {
        self.state.read().unwrap().topology().clone()
    }

    pub fn neighbors(&self, sender_id: &str) -> Vec<String> {
        let self_id = self.get_id();
        let topology = self.topology().get(&self_id).cloned().unwrap_or_default();
        topology.into_iter().filter(|v| v != sender_id).collect()
    }

    fn next_seq(&self) -> usize {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }

    pub fn next_msg_id(&self) -> usize {
        self.next_seq()
    }

    pub fn generate_id(&self) -> String {
        format!("{}-{}", self.state.read().unwrap().id, self.next_seq())
    }

    pub fn is_uninitialized(&self) -> bool {
        matches!(self.state.read().unwrap().id, IdType::None)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NodeState {
    id: IdType,
    topology: HashMap<String, Vec<String>>,
    messages: HashSet<u64>,
}

impl NodeState {
    pub fn new() -> Self {
        Self {
            id: IdType::None,
            messages: HashSet::new(),
            topology: HashMap::new(),
        }
    }

    pub fn set_messages(&mut self, msg: u64) -> bool {
        self.messages.insert(msg)
    }

    pub fn set_topology(&mut self, node_id: impl Into<String>, nodes: &[String]) {
        self.topology
            .entry(node_id.into())
            .and_modify(|v| {
                v.extend_from_slice(nodes);
            })
            .or_insert(nodes.to_owned());
    }

    pub fn messages(&self) -> Vec<u64> {
        self.messages.iter().cloned().collect()
    }

    pub fn topology(&self) -> &HashMap<String, Vec<String>> {
        &self.topology
    }
}

pub type SharedNode = Arc<RwLock<NodeState>>;
