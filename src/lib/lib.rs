extern crate irc;
extern crate reqwest;

// use irc::client::prelude::*;

pub mod middleware;
use middleware::Middleware;

pub struct Bot {
    middleware: Vec<Middleware>,
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

    pub fn use_middleware(&mut self, mw: Middleware) {
        self.middleware.push(mw);
    }
}
