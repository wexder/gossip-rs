mod node;
use crate::node::node::Node;

#[cfg(feature = "generate")]
mod generate;

#[cfg(feature = "echo")]
mod echo;

#[cfg(feature = "broadcast")]
mod broadcast;

use anyhow::Context;
use node::message::Message;
use node::server::Server;
use std::io;

fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    #[cfg(feature = "echo")]
    let mut node = echo::EchoNode::init(&mut stdin, &mut stdout)?;
    #[cfg(feature = "generate")]
    let mut node = generate::GenerateNode::init(&mut stdin, &mut stdout)?;
    #[cfg(feature = "broadcast")]
    let mut node = broadcast::BroadcastNode::init(&mut stdin, &mut stdout)?;

    drop(stdin);
    let mut server = Server::new(node);
    server.start()?;

    Ok(())
}
