mod node;
use crate::node::node::Node;

#[cfg(feature = "generate")]
mod generate;

#[cfg(feature = "echo")]
mod echo;

#[cfg(feature = "broadcast")]
mod broadcast;

use anyhow::Context;
use node::message::{Message, Request};
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
    let stdin = io::stdin();

    for line in stdin.lines() {
        let line = line?;
        if let Ok(msg) = serde_json::from_str::<Message<Request>>(line.as_str()) {
            node.step(msg.clone(), &mut stdout)
                .context("failed to step")?;
        } else {
        };
    }

    Ok(())
}
