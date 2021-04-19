use macroquad::prelude::collections::storage;

use crate::{gui_resources::GuiResources, scene_manager::SceneManager};

pub struct Game {
  scene_manager: SceneManager
}

impl Game {
  pub fn new() -> Game {
    let scene_manager = SceneManager::new();
    let gui_resources = GuiResources::new();

    storage::store(gui_resources);

    Game {
      scene_manager
    }
  }

  pub async fn run(&self) {
    self.scene_manager.render_scene().await;
  }
}
