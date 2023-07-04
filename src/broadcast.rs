use std::{
    collections::{BTreeSet, HashMap},
    io,
    marker::PhantomData,
};

use crate::node::{
    message::{Body, BodyType, Message},
    node::Node,
};
use anyhow::{Context, Ok};

#[derive(Default)]
pub struct BroadcastNode<R: io::BufRead, W: io::Write> {
    id: String,
    nodes: Vec<String>,
    messages: BTreeSet<i32>,
    neighbour_status: HashMap<String, bool>,
    uncomfirmed: HashMap<UncomfirmedMsgKey, Vec<Message>>,

    _phantom_w: PhantomData<W>,
    _phantom_r: PhantomData<R>,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct UncomfirmedMsgKey {
    node_id: String,
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
            messages: BTreeSet::new(),
            uncomfirmed: HashMap::new(),

            neighbour_status: HashMap::new(),

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
            BodyType::BatchBroadcast { messages: _ } => self.batch_broadcast(msg, output),
            BodyType::SyncTick => self.sync(msg, output),
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
        // TODO remove clone
        for node in self.nodes.clone() {
            self.neighbour_status.insert(node.clone(), false);
        }
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

    fn read(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
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
                    messages: self.messages.clone().into_iter().collect(),
                },
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }

    fn batch_broadcast(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        let messages = match msg.body.tp {
            BodyType::BatchBroadcast { messages } => messages,
            _ => anyhow::bail!("Msg has to be batch broadcast"),
        };
        self.messages.extend(messages);
        Ok(())
    }

    fn broadcast(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        let message = match msg.body.tp {
            BodyType::Broadcast { message } => message,
            _ => anyhow::bail!("Msg has to be broadcast"),
        };

        self.messages.insert(message);

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

    fn sync(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        for node in self.nodes.clone() {
            self.send_message(
                &Message {
                    src: self.id.clone(),
                    dest: node,
                    body: Body {
                        msg_id: msg.body.msg_id,
                        in_reply_to: None,
                        tp: BodyType::BatchBroadcast {
                            messages: self.messages.clone().into_iter().collect(),
                        },
                    },
                },
                output,
            )?;
        }

        Ok(())
    }

    fn broadcast_ok(&mut self, msg: Message, _: &mut W) -> anyhow::Result<()> {
        match msg.body.tp {
            BodyType::BroadcastOk => {}
            _ => anyhow::bail!("Msg has to be broadcast"),
        };

        eprintln!("BroadcastOk");

        let key = UncomfirmedMsgKey { node_id: msg.src };
        let empty = &Vec::new();
        let old = self.uncomfirmed.get(&key).unwrap_or(empty);
        let new: Vec<Message> = old
            .to_vec()
            .into_iter()
            .filter(|m| {
                m.body.msg_id
                    != msg
                        .body
                        .in_reply_to
                        .context("Failed to filter on broadcast_ok")
                        .unwrap()
            })
            .collect();

        eprintln!(
            "Ok {} {} {} {}",
            self.id,
            self.uncomfirmed.len(),
            old.len(),
            new.len()
        );
        self.uncomfirmed.insert(key, new);
        Ok(())
    }

    fn send_message(&self, msg: &Message, output: &mut W) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *output, msg).context("Cannot jsonifie reply message")?;
        output.write(b"\n").context("Cannot end the message")?;
        Ok(())
    }
}
