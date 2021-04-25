use macroquad::prelude::{KeyCode, Rect, Vec2, clear_background, collections::storage, is_key_down};
use macroquad::color::SKYBLUE;

use crate::{entities::{enemy::Enemy, entity::Direction, player::Player}, resources::Resources};

use super::Scenes;

pub struct ArenaDungeonScreen {
  pub player: Player,
  pub enemies: [Enemy; 4]
}

impl ArenaDungeonScreen {
  pub fn new() -> Self {
    let player = Player::new();
    let enemy1 = Enemy::new(Vec2::new(400., 100.));
    let enemy2 = Enemy::new(Vec2::new(600., 200.));
    let enemy3 = Enemy::new(Vec2::new(700., 50.));
    let enemy4 = Enemy::new(Vec2::new(400., 600.));

    ArenaDungeonScreen {
      player,
      enemies: [enemy1, enemy2, enemy3, enemy4]
    }
  }

  pub async fn render(&mut self) -> Option<Scenes> {
    clear_background(SKYBLUE);

    let resources = storage::get_mut::<Resources>();
    let w = resources.tiled_map.raw_tiled_map.tilewidth * resources.tiled_map.raw_tiled_map.width;
    let h = resources.tiled_map.raw_tiled_map.tileheight * resources.tiled_map.raw_tiled_map.height;
    
    resources.tiled_map.draw_tiles("ground", Rect::new(0.0, 0.0, w as f32, h as f32), None);
    resources.tiled_map.draw_tiles("wall", Rect::new(0.0, 0.0, w as f32, h as f32), None);
    
    self.player.update(&resources.collision_world);
    self.player.draw(resources.player_texture);

    for enemy in self.enemies.iter_mut() {
      enemy.update(&resources.collision_world);
      enemy.draw(resources.enemy_texture);
    };
    
    resources.tiled_map.draw_tiles("toplayer", Rect::new(0.0, 0.0, w as f32, h as f32), None);

    if is_key_down(KeyCode::W) {
      self.player.set_direction(Direction::Up);
      self.player.entity.moving = true;
    } else if is_key_down(KeyCode::S) {
      self.player.set_direction(Direction::Down);
      self.player.entity.moving = true;
    } else if is_key_down(KeyCode::A) {
      self.player.set_direction(Direction::Left);
      self.player.entity.moving = true;
    } else if is_key_down(KeyCode::D) {
      self.player.set_direction(Direction::Right);
      self.player.entity.moving = true;
    } else {
      self.player.entity.moving = false;
    }

    for enemy in self.enemies.iter_mut() {
      enemy.move_random();
    };
   
    None
  }
}
