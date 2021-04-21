use macroquad::prelude::{KeyCode, Rect, clear_background, collections::storage, is_key_down};
use macroquad::color::SKYBLUE;

use crate::{entities::player::{Direction, Player}, resources::Resources};

use super::Scenes;

pub struct ArenaDungeonScreen {
  pub player: Player
}

impl ArenaDungeonScreen {
  pub async fn new() -> Self {
    let player = Player::new().await;

    ArenaDungeonScreen {
      player
    }
  }

  pub async fn render(&mut self) -> Option<Scenes> {
    clear_background(SKYBLUE);

    let resources = storage::get_mut::<Resources>();
    let w = resources.tiled_map.raw_tiled_map.tilewidth * resources.tiled_map.raw_tiled_map.width;
    let h = resources.tiled_map.raw_tiled_map.tileheight * resources.tiled_map.raw_tiled_map.height;
    
    resources.tiled_map.draw_tiles("ground", Rect::new(0.0, 0.0, w as f32 * 2., h as f32 * 2.), None);
    resources.tiled_map.draw_tiles("wall", Rect::new(0.0, 0.0, w as f32 * 2., h as f32 * 2.), None);
    
    self.player.update();

    self.player.draw();
    
    if is_key_down(KeyCode::W) {
      self.player.set_direction(Direction::Up);
      self.player.moving = true;
    } else if is_key_down(KeyCode::S) {
      self.player.set_direction(Direction::Down);
      self.player.moving = true;
    } else if is_key_down(KeyCode::A) {
      self.player.set_direction(Direction::Left);
      self.player.moving = true;
    } else if is_key_down(KeyCode::D) {
      self.player.set_direction(Direction::Right);
      self.player.moving = true;
    } else {
      self.player.moving = false;
    }

    None
  }
}
