use crate::{gui_resources::GuiResources};
use gui::MainMenuGui;
use macroquad::{
  prelude::collections::storage,
  ui::root_ui
};

use super::Scenes;

mod gui;

pub struct MainMenuScene;

impl MainMenuScene {
  pub async fn render() -> Option<Scenes> {
    let resources = storage::get_mut::<GuiResources>();
    root_ui().push_skin(&resources.title_skin);

    if MainMenuGui::render_singleplayer_button() {
      return Some(Scenes::ArenaDungeon);
    }

    if MainMenuGui::render_multiplayerplayer_button() {
      println!("multi player clicked");
    }

    None
  }
}
