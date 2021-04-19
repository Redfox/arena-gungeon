use crate::scenes::{Scenes, main_menu::MainMenuScene};

pub struct SceneManager {
  current_scene: Scenes
}

impl SceneManager {
  pub fn new() -> Self {
    SceneManager {
      current_scene: Scenes::MainMenu
    }
  }

  pub async fn render_scene(&self) {
    match self.current_scene {
      Scenes::MainMenu => {
        MainMenuScene::render().await;
      }
    }
  }
}
