extern crate haze;
extern crate kuchiki;
// extern crate reqwest;
extern crate curl;

mod config;
mod helpers;
mod plugins;

fn main() {
    let mut bot = haze::Bot::new();
    bot.use_middleware(plugins::TitleLink);
}
