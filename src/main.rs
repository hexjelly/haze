extern crate curl;
extern crate haze;
extern crate kuchiki;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate failure;

mod config;
mod helpers;
mod plugins;

fn main() {
    pretty_env_logger::init();
    let mut bot = haze::Bot::new();
    bot.use_middleware(plugins::LinkTitle);
}
