use std::io::{self, Stdout};

use crate::node::{
    message::{Message, Reply, ReplyType, Request, RequestType},
    node::Node,
};

#[derive(Default)]
pub struct EchoNode {
    id: String,
}

impl Node<io::StdinLock<'static>, Stdout> for EchoNode {
    fn new(id: String) -> Self {
        Self { id }
    }

    fn step(&mut self, msg: Message<Request>, output: &mut Stdout) -> anyhow::Result<()> {
        match msg.body.tp {
            RequestType::Echo { echo: _ } => self.echo(msg, output),
            _ => anyhow::bail!("Unknow body type"),
        }
    }
}

impl EchoNode {
    fn echo(&self, msg: Message<Request>, output: &mut Stdout) -> anyhow::Result<()> {
        let echo = match msg.body.tp {
            RequestType::Echo { echo } => echo,
            _ => anyhow::bail!("Msg has to be echo"),
        };
        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Reply {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                tp: ReplyType::EchoOk { echo },
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }
}
