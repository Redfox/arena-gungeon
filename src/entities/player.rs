use macroquad::{
  color, 
  experimental::animation::{AnimatedSprite, Animation},
  prelude::{
    DrawTextureParams,
    Texture2D,
    Vec2,
    draw_texture_ex,
    vec2
  }
};
use macroquad_platformer::World;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
  Left = 0,
  Right = 1,
  Down = 2,
  Up = 3,
}

pub struct Player {
  pub position: Vec2,
  pub moving: bool,
  sprite: AnimatedSprite,
  direction: Direction,
}

impl Player {
  pub fn new() -> Self {
    let sprite = AnimatedSprite::new(32, 32, &[
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
      position: vec2(100., 100.),
      direction: Direction::Down,
      sprite,
      moving: false,
    }
  }

  pub fn draw(&mut self, player_texture: Texture2D) {
    draw_texture_ex(
      player_texture, 
      self.position.x, 
      self.position.y, 
      color::WHITE,
      DrawTextureParams {
        source: Some(self.sprite.frame().source_rect),
        dest_size: Some(self.sprite.frame().dest_size),
        ..Default::default()
      }
    )
  }

  pub fn update(&mut self, collision_world: &World) {
    self.sprite.playing = self.moving;

    self.sprite.set_animation(self.direction as usize);
    self.sprite.update();

    let n = 5.;

    if self.moving {
      let pos = match self.direction {
        Direction::Up => {
          if collision_world.solid_at(vec2(self.position.x, self.position.y - n)) {
            vec2(0., 0.)
          } else {
            vec2(0., -5.)
          }
        },
        Direction::Down  => {
          if collision_world.solid_at(vec2(self.position.x, self.position.y + 32.))  {
            vec2(0., 0.)
          } else {
            vec2(0., 5.)
          }
        },
        Direction::Left  => {
          if collision_world.solid_at(vec2(self.position.x - n, self.position.y))  {
            vec2(0., 0.)
          } else {
            vec2(-5., 0.)
          }
        },
        Direction::Right  => {
          if collision_world.solid_at(vec2(self.position.x + 32., self.position.y))  {
            vec2(0., 0.)
          } else {
            vec2(5., 0.)
          }
        },
      };

      self.position = vec2(self.position.x + pos.x, self.position.y + pos.y);
    }
  }

  pub fn set_direction(&mut self, direction: Direction) {
    self.direction = direction;
  }
}
