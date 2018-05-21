extern crate haze;
extern crate kuchiki;
extern crate reqwest;

mod plugins;
mod config;

fn main() {
    let mut bot = haze::Bot::new();
    bot.use_middleware(plugins::TitleLink);
}
