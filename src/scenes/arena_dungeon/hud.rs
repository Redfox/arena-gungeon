use macroquad::{
  prelude::{draw_rectangle, screen_height},
  color,
};

#[derive(Copy, Clone)]
pub struct ArenaHUD;

impl ArenaHUD {
  pub fn new() -> Self {
    ArenaHUD
  }

  pub fn render_player_hud(self) {
    self.render_player_health_bar();
    self.render_player_skills();
  }

  fn render_player_health_bar(self) {
    draw_rectangle(10., 10., 200., 30., color::BLACK);
    draw_rectangle(13., 13., 194., 24., color::RED);
  }

  fn render_player_skills(self) {
    let w_pos = 50.;
    let h_pos = screen_height() - (screen_height() / 10.);

    let skill_count = 3;

    for n in 0..skill_count {
      draw_rectangle(w_pos + ((50. + 10.) * n as f32), h_pos, 50., 50., color::BLACK);
      draw_rectangle(w_pos + ((50. + 10.) * n as f32) + 3., h_pos + 3., 50. - 6., 50. -6., color::BLUE);
    }
  } 
}
