use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T> {
    pub src: String,
    pub dest: String,
    pub body: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub struct Request {
    pub msg_id: i32,
    #[serde(flatten)]
    pub tp: RequestType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub struct Reply {
    pub msg_id: i32,
    pub in_reply_to: i32,
    #[serde(flatten)]
    pub tp: ReplyType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    Echo {
        echo: Option<String>,
    },
    Generate,
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    Broadcast {
        message: i32,
    },
    Read,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ReplyType {
    InitOk,
    EchoOk { echo: Option<String> },
    GenerateOk { id: Uuid },
    BroadcastOk,
    ReadOk { messages: Vec<i32> },
    TopologyOk,
}
