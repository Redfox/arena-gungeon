use macroquad::prelude::clear_background;
use macroquad::color::SKYBLUE;

use super::Scenes;

pub struct ArenaDungeonScreen;

impl ArenaDungeonScreen {
  pub async fn render() -> Option<Scenes> {
    clear_background(SKYBLUE);

    None
  }
}
