use macroquad::prelude::next_frame;

use crate::scenes::{
  Scenes,
  arena_dungeon::ArenaDungeonScreen,
  main_menu::MainMenuScene
};

pub struct SceneManager {
  pub current_scene: Scenes
}

impl SceneManager {
  pub fn new() -> Self {
    SceneManager {
      current_scene: Scenes::MainMenu
    }
  }

  pub fn change_scene(&mut self, scene: Scenes) {
    self.current_scene = scene;
  }

  pub async fn render_scene(&mut self) {
    loop {
      let scene_option = match self.current_scene {
        Scenes::MainMenu => {
          MainMenuScene::render().await
        }
        Scenes::ArenaDungeon => {
          ArenaDungeonScreen::render().await
        }
      };

      match scene_option {
        Some(scene) => self.change_scene(scene),
        None    => {}
      }

      next_frame().await;
    }
  }
}
