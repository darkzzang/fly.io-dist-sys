use crate::error::Error as AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}

impl Message {
    pub fn into_reply(self, msg_id: Option<usize>, payload: Payload) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
            body: MessageBody {
                msg_id,
                in_reply_to: self.body.msg_id,
                payload,
            },
        }
    }
    pub fn is_error(&self) -> bool {
        self.body.is_error()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

impl MessageBody {
    pub fn is_error(&self) -> bool {
        self.payload.is_error()
    }

    pub fn raw(&self) -> Result<String, AppError> {
        serde_json::to_string(self)
            .map_err(|err| AppError::CustomError((1002, Some(err.to_string()))))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Error {
        code: i32,
        text: String,
    },
    Generate,
    GenerateOk {
        id: String,
    },
    Broadcast {
        message: u64,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<u64>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    Gossip {
        message: u64,
    },
    None,
    #[serde(other)]
    UnknownMessage,
}

impl Payload {
    pub fn is_none(&self) -> bool {
        matches!(self, Payload::None)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Payload::Error { .. })
    }
}

impl From<AppError> for Payload {
    fn from(err: AppError) -> Self {
        Payload::Error {
            code: err.code(),
            text: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_message_body_serialization() {
        let msg = MessageBody {
            msg_id: Some(1),
            in_reply_to: None,
            payload: Payload::Init {
                node_id: "node1".to_string(),
                node_ids: vec!["node2".to_string(), "node3".to_string()],
            },
        };
        let expected = json!({
            "type": "init",
            "msg_id": 1,
            "node_id": "node1",
            "node_ids": ["node2", "node3"]
        });
        let actual = serde_json::to_value(msg).unwrap();
        assert_eq!(actual, expected);
    }
}
