use crate::gui_resources::GuiResources;
use gui::MainMenuGui;
use macroquad::{
  prelude::collections::storage,
  ui::root_ui,
  window::next_frame
};

mod gui;

pub struct MainMenuScene;

impl MainMenuScene {
  pub async fn render() {
    let resources = storage::get_mut::<GuiResources>();
    root_ui().push_skin(&resources.title_skin);

    loop {
      if MainMenuGui::render_singleplayer_button() {
        println!("single player clicked");
      }

      if MainMenuGui::render_multiplayerplayer_button() {
        println!("multi player clicked");
      }

      next_frame().await;
    }
  }
}
