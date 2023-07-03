use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use std::{io, time::SystemTime};

use anyhow::Context;

use super::message::BodyType;
use super::{
    message::{Body, Message},
    node::Node,
};

pub struct Server<T: Node<io::StdinLock<'static>, io::Stdout>> {
    node: T,
}

impl<T: Node<io::StdinLock<'static>, io::Stdout>> Server<T> {
    pub fn new(node: T) -> Self {
        Self { node }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let mut stdout = io::stdout();
        let stdin = io::stdin();

        let (tx, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        let main_tx = tx.clone();
        thread::spawn(move || {
            for line in stdin.lines() {
                let line = line.unwrap();
                if let Ok(msg) = serde_json::from_str::<Message>(line.as_str()) {
                    main_tx.send(msg).unwrap();
                }
            }
        });

        let ticker_tx = tx.clone();
        let id = self.node.get_id();
        thread::spawn(move || {
            let seconds = Duration::from_millis(500);

            loop {
                let start = SystemTime::now();
                thread::sleep(Duration::from_millis(500));
                match start.elapsed() {
                    Ok(elapsed) if elapsed > seconds => {
                        ticker_tx
                            .send(Message {
                                src: "ticker".to_string(),
                                dest: id.clone(),
                                body: Body {
                                    msg_id: 0,
                                    in_reply_to: None,
                                    tp: BodyType::Sync,
                                },
                            })
                            .unwrap();
                    }
                    _ => (),
                }
            }
        });

        for msg in receiver.iter() {
            match msg.body.in_reply_to {
                Some(_) => {
                    self.node
                        .reply(msg.clone(), &mut stdout)
                        .context("failed to step")
                        .unwrap();
                }
                None => {
                    self.node
                        .step(msg.clone(), &mut stdout)
                        .context("failed to step")
                        .unwrap();
                }
            }
        }

        Ok(())
    }
}
