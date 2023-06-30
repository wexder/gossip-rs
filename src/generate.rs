use std::io::{self, Stdout};

use crate::node::{
    message::{Message, Reply, ReplyType, Request, RequestType},
    node::Node,
};
use uuid::Uuid;

#[derive(Default)]
pub struct GenerateNode {
    id: String,
}

impl Node<io::StdinLock<'static>, Stdout> for GenerateNode {
    fn new(id: String) -> Self {
        Self { id }
    }

    fn step(&mut self, msg: Message<Request>, output: &mut Stdout) -> anyhow::Result<()> {
        match msg.body.tp {
            RequestType::Generate => self.generate(msg, output),
            _ => anyhow::bail!("Unknow body type"),
        }
    }
}

impl GenerateNode {
    fn generate(&self, msg: Message<Request>, output: &mut Stdout) -> anyhow::Result<()> {
        match msg.body.tp {
            RequestType::Generate => {}
            _ => anyhow::bail!("Msg has to be generate"),
        };
        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Reply {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                tp: ReplyType::GenerateOk { id: Uuid::new_v4() },
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }
}
