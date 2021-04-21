use macroquad::prelude::{
  clear_background,
  draw_text,
  screen_height,
  screen_width,
  get_time,
  collections::storage,
  coroutines::start_coroutine,
  next_frame,
};
use macroquad::color;

use crate::{resources::Resources, scenes::{
  Scenes,
  arena_dungeon::ArenaDungeonScreen,
  main_menu::MainMenuScene,
}};

pub struct SceneManager {
  pub current_scene: Scenes
}

impl SceneManager {
  pub fn new() -> Self {
    SceneManager {
      current_scene: Scenes::MainMenu,
    }
  }

  pub fn change_scene(&mut self, scene: Scenes) {
    self.current_scene = scene;
  }

  pub async fn render_scene(&mut self) {
    let resources_loading = start_coroutine(async move {
      let resources = Resources::new().await.unwrap();
      storage::store(resources);
    });

    let mut arena_dungeon_scene = ArenaDungeonScreen::new().await;

    loop {
      let scene_option = match self.current_scene {
        Scenes::MainMenu => {
          MainMenuScene::render().await
        }
        Scenes::ArenaDungeon => {
          while !resources_loading.is_done() {
            clear_background(color::BLACK);
            draw_text(
              &format!(
                  "Loading resources {}",
                  ".".repeat(((get_time() * 2.0) as usize) % 4)
              ),
              screen_width() / 2.0 - 160.0,
              screen_height() / 2.0,
              40.,
              color::WHITE,
            );

            next_frame().await;
          }
          
          arena_dungeon_scene.render().await
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
