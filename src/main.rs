mod middleware;

extern crate irc;
use irc::client::prelude::*;

fn main() {
    let mut reactor = IrcReactor::new().unwrap();
    let config = Config::load("src/config.toml").unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.register_client_with_handler(client, |client, message| {
        print!("{}", message);
        // And here we can do whatever we want with the messages.
        Ok(())
    });

    reactor.run().unwrap();
}
