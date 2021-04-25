use macroquad::{experimental::animation::{AnimatedSprite, Animation}, prelude::{Texture2D, vec2}};
use macroquad_platformer::World;

use super::entity::{Direction, Entity};

pub struct Player {
  pub entity: Entity,
}

impl Player {
  pub fn new() -> Self {
    let tile_width = 18;
    let tile_height = 20;
    let sprite = AnimatedSprite::new(tile_width, tile_height, &[
      Animation {
        name: "up".to_string(),
        row: 0,
        frames: 4,
        fps: 12,
      },
      Animation {
        name: "down".to_string(),
        row: 1,
        frames: 4,
        fps: 12,
      },
      Animation {
        name: "left".to_string(),
        row: 2,
        frames: 4,
        fps: 12,
      },
      Animation {
        name: "right".to_string(),
        row: 3,
        frames: 4,
        fps: 12,
      }
    ], true);
    
    Player {
      entity: Entity {
        position: vec2(100., 100.),
        direction: Direction::Down,
        sprite,
        moving: false,
        speed: 5,
        tile_height,
        tile_width
      }
    }
  }

  pub fn draw(&mut self, texture: Texture2D) {
    self.entity.draw(texture);
  }

  pub fn update(&mut self, collision_world: &World) {
    self.entity.update(collision_world);
  }

  pub fn set_direction(&mut self, direction: Direction) {
    self.entity.direction = direction;
  }
}
