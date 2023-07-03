use std::{io, marker::PhantomData};

use crate::node::{
    message::{Body, BodyType, Message},
    node::Node,
};
use anyhow::Context;

#[derive(Default)]
pub struct BroadcastNode<R: io::BufRead, W: io::Write> {
    id: String,
    nodes: Vec<String>,
    messages: Vec<i32>,
    uncomfirmed: Vec<Message>,

    _phantom_w: PhantomData<W>,
    _phantom_r: PhantomData<R>,
}

impl<R, W> Node<R, W> for BroadcastNode<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    fn new(id: String) -> Self {
        Self {
            id,
            nodes: Vec::new(),
            messages: Vec::new(),
            uncomfirmed: Vec::new(),

            _phantom_w: PhantomData,
            _phantom_r: PhantomData,
        }
    }

    fn step(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        if msg.dest != self.id {
            return Ok(());
        }
        match msg.body.tp {
            BodyType::Topology { topology: _ } => self.topology(msg, output),
            BodyType::Read => self.read(msg, output),
            BodyType::Broadcast { message: _ } => self.broadcast(msg, output),
            BodyType::Sync => {
                // TOOD remove clone
                for msg in self.uncomfirmed.clone() {
                    self.send_message(&msg, output)?;
                    self.uncomfirmed.push(msg);
                }
                Ok(())
            }
            _ => anyhow::bail!("Unknow body type"),
        }?;

        Ok(())
    }

    fn reply(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        if msg.dest != self.id {
            return Ok(());
        }

        match msg.body.tp {
            BodyType::BroadcastOk => self.broadcast_ok(msg, output),
            _ => anyhow::bail!("Unknow body type"),
        }
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl<R, W> BroadcastNode<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    fn topology(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        let topology = match msg.body.tp {
            BodyType::Topology { topology } => topology,
            _ => anyhow::bail!("Msg has to be topology"),
        };
        self.nodes = topology.get(&self.id).unwrap().to_vec();
        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: Some(msg.body.msg_id),
                tp: BodyType::TopologyOk,
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }

    fn read(&self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        match msg.body.tp {
            BodyType::Read => {}
            _ => anyhow::bail!("Msg has to be read"),
        };
        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: Some(msg.body.msg_id),
                tp: BodyType::ReadOk {
                    messages: self.messages.clone(),
                },
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }

    fn broadcast(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        let message = match msg.body.tp {
            BodyType::Broadcast { message } => message,
            _ => anyhow::bail!("Msg has to be broadcast"),
        };
        self.messages.push(message);

        // TOOD remove clone
        for node in self.nodes.clone() {
            if node == msg.src {
                continue;
            }
            let resp = Message {
                src: self.id.clone(),
                dest: node.clone(),
                body: Body {
                    msg_id: msg.body.msg_id,
                    in_reply_to: None,
                    tp: BodyType::Broadcast { message },
                },
            };

            self.send_message(&resp, output)?;
            self.uncomfirmed.push(resp);
        }

        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: Some(msg.body.msg_id),
                tp: BodyType::BroadcastOk,
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }

    fn broadcast_ok(&mut self, msg: Message, _: &mut W) -> anyhow::Result<()> {
        match msg.body.tp {
            BodyType::BroadcastOk => {}
            _ => anyhow::bail!("Msg has to be broadcast"),
        };

        self.uncomfirmed = self
            .uncomfirmed
            // TOOD remove clone
            .clone()
            .into_iter()
            .filter(|m| m.body.msg_id != msg.body.in_reply_to.unwrap())
            .collect();
        Ok(())
    }

    fn send_message(&self, msg: &Message, output: &mut W) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *output, msg).context("Cannot jsonifie reply message")?;
        output.write(b"\n").context("Cannot end the message")?;
        Ok(())
    }
}
