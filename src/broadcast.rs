use std::io::{self, Stdout};

use crate::node::{
    message::{Message, Reply, ReplyType, Request, RequestType},
    node::Node,
};

#[derive(Default)]
pub struct BroadcastNode {
    id: String,
    nodes: Vec<String>,
    messages: Vec<i32>,
}

impl Node<io::StdinLock<'static>, Stdout> for BroadcastNode {
    fn new(id: String) -> Self {
        Self {
            id,
            nodes: Vec::new(),
            messages: Vec::new(),
        }
    }

    fn step(&mut self, msg: Message<Request>, output: &mut Stdout) -> anyhow::Result<()> {
        match msg.body.tp {
            RequestType::Topology { topology: _ } => self.topology(msg, output),
            RequestType::Read => self.read(msg, output),
            RequestType::Broadcast { message: _ } => self.broadcast(msg, output),
            _ => anyhow::bail!("Unknow body type"),
        }
    }
}

impl BroadcastNode {
    fn topology(&mut self, msg: Message<Request>, output: &mut Stdout) -> anyhow::Result<()> {
        let topology = match msg.body.tp {
            RequestType::Topology { topology } => topology,
            _ => anyhow::bail!("Msg has to be topology"),
        };
        self.nodes = topology.get(&self.id).unwrap().to_vec();
        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Reply {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                tp: ReplyType::TopologyOk,
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }

    fn read(&self, msg: Message<Request>, output: &mut Stdout) -> anyhow::Result<()> {
        match msg.body.tp {
            RequestType::Read => {}
            _ => anyhow::bail!("Msg has to be read"),
        };
        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Reply {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                tp: ReplyType::ReadOk {
                    messages: self.messages.clone(),
                },
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }

    fn broadcast(&mut self, msg: Message<Request>, output: &mut Stdout) -> anyhow::Result<()> {
        let message = match msg.body.tp {
            RequestType::Broadcast { message } => message,
            _ => anyhow::bail!("Msg has to be broadcast"),
        };
        self.messages.push(message);

        for node in self.nodes.clone() {
            if node == msg.src {
                continue;
            }
            let resp = Message {
                src: self.id.clone(),
                dest: node.clone(),
                body: Request {
                    msg_id: msg.body.msg_id,
                    tp: RequestType::Broadcast { message },
                },
            };

            self.send_message(&resp, output)?;
        }

        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Reply {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                tp: ReplyType::BroadcastOk,
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }
}
