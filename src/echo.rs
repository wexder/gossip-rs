use std::{io, marker::PhantomData};

use crate::node::{
    message::{Body, BodyType, Message},
    node::Node,
};

#[derive(Default)]
pub struct EchoNode<R: io::BufRead, W: io::Write> {
    id: String,
    _phantom_w: PhantomData<W>,
    _phantom_r: PhantomData<R>,
}

impl<R, W> Node<R, W> for EchoNode<R, W>
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
            BodyType::Echo { echo: _ } => self.echo(msg, output),
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

impl<R, W> EchoNode<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    fn echo(&self, msg: Message, output: &mut W) -> anyhow::Result<()> {
        let echo = match msg.body.tp {
            BodyType::Echo { echo } => echo,
            _ => anyhow::bail!("Msg has to be echo"),
        };
        let resp = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: Some(msg.body.msg_id),
                tp: BodyType::EchoOk { echo },
            },
        };

        self.send_message(&resp, output)?;
        Ok(())
    }
}
