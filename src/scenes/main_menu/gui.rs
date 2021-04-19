use macroquad::{
  prelude::{vec2, screen_height, screen_width},
  ui::{widgets, root_ui}
};

pub struct MainMenuGui;

impl MainMenuGui {
  pub fn render_singleplayer_button() -> bool {
    let h_pos = screen_height() - (screen_height() / 4.);
    let w_pos = screen_width() / 6.;

    widgets::Button::new("Single Player")
      .size(vec2(230., 70.))
      .position(vec2(
        w_pos, h_pos
      ))
      .ui(&mut *root_ui())
  }

  pub fn render_multiplayerplayer_button() -> bool {
    let h_pos = screen_height() - (screen_height() / 4.);
    let w_pos = screen_width() / 2.;
    
    widgets::Button::new("Multi Player")
      .size(vec2(230., 70.))
      .position(vec2(
        w_pos, h_pos
      ))
      .ui(&mut *root_ui())
  }
}
