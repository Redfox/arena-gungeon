use crate::scene::Scene;
use macroquad::{
  window::next_frame
};

pub struct SceneManager {
  current_scene: Scene
}

impl SceneManager {
  pub fn new() -> Self {
    SceneManager {
      current_scene: Scene::MainMenu
    }
  }

  pub async fn render_scene(&self) {
    match self.current_scene {
      Scene::MainMenu => {
        //render Main Menu

        next_frame().await;
      }
    }
  }
}
