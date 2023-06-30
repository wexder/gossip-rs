use anyhow::Context;
use serde::Serialize;

use crate::node::message::{Message, Reply, Request};

use std::{io, sync::Mutex};

use super::message::RequestType;

pub trait Node<R, W>: Sized
where
    R: io::BufRead,
    W: io::Write,
{
    fn init(stdin: &mut R, output: &mut W) -> anyhow::Result<Self> {
        let mut buf = String::default();

        stdin
            .read_line(&mut buf)
            .context("Cannot read first line")?;

        let msg: Message<Request> =
            serde_json::from_str(&buf).context("Cannot parse init message")?;

        let node_id = match msg.body.tp {
            RequestType::Init {
                node_id,
                node_ids: _,
            } => {
                let resp = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Reply {
                        msg_id: msg.body.msg_id + 1,
                        in_reply_to: msg.body.msg_id,
                        tp: super::message::ReplyType::InitOk,
                    },
                };
                let resp = serde_json::to_string(&resp)
                    .context("Cannot jsonifie init_ok reply message")?;
                output
                    .write((resp + "\n").as_bytes())
                    .context("Cannot end the message")?;
                drop(output);

                node_id
            }
            _ => panic!("First message has to be of type: \"init\""),
        };

        Ok(Self::new(node_id))
    }

    fn send_message<T: Serialize>(&self, msg: &Message<T>, output: &mut W) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *output, msg).context("Cannot jsonifie reply message")?;
        output.write(b"\n").context("Cannot end the message")?;
        Ok(())
    }

    fn new(id: String) -> Self;
    fn step(&mut self, msg: Message<Request>, output: &mut W) -> anyhow::Result<()>;
}
