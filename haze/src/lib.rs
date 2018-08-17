extern crate irc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

// use irc::client::prelude::*;

pub mod middleware;
use middleware::{Message, Middleware};

pub struct Bot {
    middleware: Vec<Box<Middleware>>,
}

impl Bot {
    pub fn new() -> Self {
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
        Bot { middleware: vec![] }
    }

    pub fn use_middleware<M: 'static + Middleware>(&mut self, mw: M) {
        info!("Using middleware: {}", mw.name());
        self.middleware.push(Box::new(mw));
    }

    pub fn handle_msg(&self, mut msg: Message) {
        for mw in &self.middleware {
            debug!(
                "Processing message: {:?} using \"{}\" middleware",
                msg,
                mw.name()
            );
            mw.process(&mut msg).unwrap();
        }
    }

    pub fn list_middleware(&self) -> Vec<String> {
        let mut names = vec![];
        for mw in &self.middleware {
            names.push(mw.name());
        }
        names
    }
}
