use std::{io, marker::PhantomData};

use crate::node::{
    message::{Body, BodyType, Message},
    node::Node,
};
use uuid::Uuid;

#[derive(Default)]
pub struct GenerateNode<R: io::BufRead, W: io::Write> {
    id: String,

    _phantom_w: PhantomData<W>,
    _phantom_r: PhantomData<R>,
}

impl<R, W> Node<R, W> for GenerateNode<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    fn new(id: String) -> Self {
        Self {
            id,
            _phantom_w: PhantomData,
            _phantom_r: PhantomData,
        }
    }

    fn step(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        match msg.body.tp {
            BodyType::Generate => self.generate(msg, output),
            _ => Ok(()),
        }
    }

    fn reply(&mut self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        Ok(())
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl<R, W> GenerateNode<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    fn generate(&self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        match msg.body.tp {
            BodyType::Generate => {}
            _ => anyhow::bail!("Msg has to be generate"),
        };
        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: Some(msg.body.msg_id),
                tp: BodyType::GenerateOk { id: Uuid::new_v4() },
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }
}
