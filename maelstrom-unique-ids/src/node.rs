use serde::{Deserialize, Serialize};
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
}

impl Node {
    pub fn new() -> Self {
        Self {
            id: IdType::None,
            counter: AtomicUsize::new(1),
        }
    }

    pub fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    pub fn generate_id(&self) -> String {
        let count = self.counter.fetch_add(1, Ordering::Relaxed);
        format!("{}-{}", self.id, count)
    }
}
