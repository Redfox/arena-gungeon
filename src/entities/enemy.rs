use macroquad::prelude::{Texture2D, Vec2, animation::{AnimatedSprite, Animation}, get_time};
use macroquad_platformer::World;
use rand::Rng;

use super::entity::{Direction, Entity};

pub struct Enemy {
  pub entity: Entity,
  last_dir: usize
}

impl Enemy {
  pub fn new(pos: Vec2) -> Self {
    let tile_width = 16;
    let tile_height = 16;
    let sprite = AnimatedSprite::new(16, 16, &[
      Animation {
        name: "idle".to_string(),
        row: 0,
        frames: 4,
        fps: 12,
      },
      Animation {
        name: "idle".to_string(),
        row: 0,
        frames: 4,
        fps: 12,
      },
      Animation {
        name: "idle".to_string(),
        row: 0,
        frames: 4,
        fps: 12,
      },
      Animation {
        name: "idle".to_string(),
        row: 0,
        frames: 4,
        fps: 12,
      }
    ], true);

    Enemy {
      entity: Entity {
        position: pos,
        direction: Direction::Down,
        sprite,
        moving: false,
        speed: 5,
        tile_width,
        tile_height
      },
      last_dir: 0
    }
  }

  pub fn draw(&mut self, texture: Texture2D) {
    self.entity.draw(texture);
  }

  pub fn update(&mut self, collision_world: &World) {
    self.entity.update(collision_world);
  }

  pub fn move_random(&mut self) {
    self.entity.moving = true;

    let time = ((get_time() * 2.0) as usize) % 5;
    
    if self.last_dir != time {
      self.last_dir = time;

      let number = rand::thread_rng().gen_range(0..=3);

      let random_dir: Option<Direction> = match number {
        0 => Some(Direction::Down),
        1 => Some(Direction::Up),
        2 => Some(Direction::Left),
        3 => Some(Direction::Right),
        _ => None
      };

      match random_dir {
        Some(dir) => self.entity.direction = dir,
        None => {}
      }
      
    }

  }
}
