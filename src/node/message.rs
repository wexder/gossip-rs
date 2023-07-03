use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub struct Body {
    pub msg_id: i32,
    pub in_reply_to: Option<i32>,
    #[serde(flatten)]
    pub tp: BodyType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BodyType {
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

    InitOk,
    EchoOk {
        echo: Option<String>,
    },
    GenerateOk {
        id: Uuid,
    },
    BroadcastOk,
    ReadOk {
        messages: Vec<i32>,
    },
    TopologyOk,

    Sync,
}
