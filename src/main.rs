// main.rs
use ggez::{conf, event, ContextBuilder};
use std::path;

mod components;
mod systems;
mod game;

use game::Game;

pub fn main() -> ggez::GameResult {
    let context_builder = ContextBuilder::new("diablo-5", "shabixiaomi")
        .window_setup(conf::WindowSetup::default().title("Diablo 5"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;
    let game = Game::new();
    
    event::run(context, event_loop, game)
}
