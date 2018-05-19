extern crate haze;

mod plugins;

fn main() {
    let mut bot = haze::Bot::new();
    bot.use_middleware(plugins::title());
}
