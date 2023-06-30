use std::{collections::HashMap, io};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Body {
    Init {
        msg_id: Option<i32>,
        node_id: String,
        node_ids: Vec<String>,
    },
    Echo {
        msg_id: Option<i32>,
        echo: Option<String>,
    },
    Generate {
        msg_id: Option<i32>,
    },
    Topology {
        msg_id: Option<i32>,
        topology: HashMap<String, Vec<String>>,
    },
    Broadcast {
        msg_id: Option<i32>,
        message: i32,
    },
    Read {
        msg_id: Option<i32>,
    },

    InitOk {
        msg_id: Option<i32>,
        in_reply_to: i32,
    },
    EchoOk {
        msg_id: Option<i32>,
        echo: Option<String>,
        in_reply_to: i32,
    },
    GenerateOk {
        msg_id: Option<i32>,
        in_reply_to: i32,
        id: Uuid,
    },

    BroadcastOk {
        msg_id: Option<i32>,
        in_reply_to: i32,
    },
    ReadOk {
        msg_id: Option<i32>,
        in_reply_to: i32,
        messages: Vec<i32>,
    },
    TopologyOk {
        msg_id: Option<i32>,
        in_reply_to: i32,
    },
}

#[derive(Default)]
struct Node {
    id: String,
    topology_state: TopologyState,
}

#[derive(Default)]
struct TopologyState {
    nodes: Vec<String>,
    messages: Vec<i32>,
}

impl Node {
    fn step<W>(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()>
    where
        W: io::Write,
    {
        match msg.body {
            Body::Init {
                msg_id,
                node_id,
                node_ids: _,
            } => {
                self.id = node_id;
                let resp = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body::InitOk {
                        msg_id,
                        in_reply_to: msg_id.unwrap(),
                    },
                };
                serde_json::to_writer(&mut *output, &resp)?;
                output.write(b"\n")?;
            }
            Body::Echo { msg_id, echo } => {
                let resp = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body::EchoOk {
                        msg_id,
                        echo,
                        in_reply_to: msg_id.unwrap(),
                    },
                };

                serde_json::to_writer(&mut *output, &resp)?;
                output.write(b"\n")?;
            }
            Body::Generate { msg_id } => {
                let resp = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body::GenerateOk {
                        id: Uuid::new_v4(),
                        msg_id,
                        in_reply_to: msg_id.unwrap(),
                    },
                };

                serde_json::to_writer(&mut *output, &resp)?;
                output.write(b"\n")?;
            }
            Body::Broadcast { msg_id, message } => {
                for node in self.topology_state.nodes.clone() {
                    if node == msg.src {
                        continue;
                    }
                    self.topology_state.messages.push(message);
                    let resp = Message {
                        src: self.id.clone(),
                        dest: node,
                        body: Body::Broadcast { msg_id, message },
                    };

                    serde_json::to_writer(&mut *output, &resp)?;
                    output.write(b"\n")?;
                }

                let resp = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body::BroadcastOk {
                        msg_id,
                        in_reply_to: msg_id.unwrap(),
                    },
                };

                serde_json::to_writer(&mut *output, &resp)?;
                output.write(b"\n")?;
            }
            Body::Topology { msg_id, topology } => {
                self.topology_state.nodes = topology.get(&self.id).unwrap().to_vec();
                let resp = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body::TopologyOk {
                        msg_id,
                        in_reply_to: msg_id.unwrap(),
                    },
                };

                serde_json::to_writer(&mut *output, &resp)?;
                output.write(b"\n")?;
            }
            Body::Read { msg_id } => {
                let resp = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body::ReadOk {
                        msg_id,
                        in_reply_to: msg_id.unwrap(),
                        messages: self.topology_state.messages.clone(),
                    },
                };

                serde_json::to_writer(&mut *output, &resp)?;
                output.write(b"\n")?;
            }

            _ => {}
        };
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();

    let mut node = Node::default();
    for line in stdin.lines() {
        let line = line?;
        let msg: Message = serde_json::from_str(line.as_str())?;
        node.step(msg.clone(), &mut stdout)
            .context("failed to step")?;
    }
    Ok(())
}
