extern crate irc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

// use irc::client::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub mod middleware;
use middleware::{IrcMessage, Message, Middleware};

pub struct Bot {
    middleware: Vec<Box<Middleware>>,
    tx: Sender<Message>,
    rx: Receiver<Message>,
    running: bool,
}

impl Bot {
    pub fn new() -> Self {
        let (tx, rx) = channel();

        Bot {
            middleware: vec![],
            tx,
            rx,
            running: false,
        }
    }

    pub fn use_middleware<M: 'static + Middleware>(&mut self, mw: M) {
        info!("Using middleware: {}", mw.name());
        self.middleware.push(Box::new(mw));
    }

    pub fn handle_msg(&self, msg: Message) {
        let sync_msg = Arc::new(Mutex::new(msg));

        for n in 0..10 {
            println!("{}", n);
            let ms = time::Duration::from_millis(500);
            thread::sleep(ms);
            let (msg, tx) = (sync_msg.clone(), self.tx.clone());
            if n == 2 {
                thread::spawn(move || {
                    let _eh = msg;

                    let irc_msg = IrcMessage::new(
                        Some("haze"),
                        "PRIVMSG",
                        vec!["#channel"],
                        Some("Sending this reply"),
                    ).unwrap();

                    let reply = Message::from(&irc_msg);
                    tx.send(reply).unwrap();
                });
            }
        }

        // for mw in &self.middleware {
        //     debug!(
        //         "Processing message: {:?} using \"{}\" middleware",
        //         msg,
        //         mw.name()
        //     );
        //     mw.process(&mut msg).unwrap();
        // }
    }

    pub fn list_middleware(&self) -> Vec<String> {
        let mut names = vec![];
        for mw in &self.middleware {
            names.push(mw.name());
        }
        names
    }

    pub fn run(&self) {
        // let mut reactor = IrcReactor::new().unwrap();
        // let config = Config::load("src/config.toml").unwrap();
        // let client = reactor.prepare_client_and_connect(&config).unwrap();
        // client.identify().unwrap();
        //
        // reactor.register_client_with_handler(client, |client, message| {
        //     print!("{}", message);
        //     // And here we can do whatever we want with the messages.
        //     Ok(())
        // });
        //
        // reactor.run().unwrap();
        // loop {
        let irc_msg = IrcMessage::new(
            Some("haze"),
            "PRIVMSG",
            vec!["#channel"],
            Some("Sending this to be handled"),
        ).unwrap();

        let mut haze_msg = Message::from(&irc_msg);

        self.handle_msg(haze_msg);
        // }
    }
}
