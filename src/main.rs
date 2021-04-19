use constants::{WINDOW_HEIGHT, WINDOW_WIDHT, WINDOW_TITLE};

use game::Game;
use macroquad::prelude::Conf;

mod game;
mod scenes;
mod scene_manager;
mod gui_resources;
mod constants;

fn window_conf() -> Conf {
  Conf {
    window_title: String::from(WINDOW_TITLE),
    fullscreen: false,
    window_height: WINDOW_HEIGHT,
    window_width: WINDOW_WIDHT,
    window_resizable: false,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let game = Game::new();

  game.run().await;
}
