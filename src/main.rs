use scene_manager::SceneManager;
use macroquad::prelude::Conf;
use constants::{WINDOW_HEIGHT, WINDOW_WIDHT, WINDOW_TITLE};

mod scene;
mod scene_manager;
mod constants;

fn window_conf() -> Conf {
  Conf {
    window_title: String::from(WINDOW_TITLE),
    fullscreen: false,
    window_height: WINDOW_HEIGHT,
    window_width: WINDOW_WIDHT,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let scene_manager = SceneManager::new();

  loop {
    scene_manager.render_scene().await;
  }
}
