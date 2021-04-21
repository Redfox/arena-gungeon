use macroquad::{color, experimental::{
    animation::{AnimatedSprite, Animation},
  }, prelude::{DrawTextureParams, Texture2D, Vec2, draw_texture_ex, load_texture, vec2}};

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
  player_texture: Texture2D,
  sprite: AnimatedSprite,
  direction: Direction,
}

impl Player {
  pub async fn new() -> Self {
    let sprite = AnimatedSprite::new(16, 16, &[
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
    
    let player_texture = load_texture("resources/player-spritesheet.png")
    .await
    .expect("Fail to load playersheet");
    
    Player {
      position: vec2(100., 100.),
      direction: Direction::Down,
      sprite,
      player_texture,
      moving: false,
    }
  }

  pub fn draw(&mut self) {
    if self.moving {
      self.sprite.update();
    }

    draw_texture_ex(
      self.player_texture, 
      self.position.x, 
      self.position.y, 
      color::WHITE,
      DrawTextureParams {
        source: Some(self.sprite.frame().source_rect),
        dest_size: Some(vec2((16 as f32) * 3., (16  as f32)* 3.)),
        ..Default::default()
      }
    )
  }

  pub fn update(&mut self) {
    self.sprite.set_animation(self.direction as usize);

    if self.moving {
      let pos = match self.direction {
        Direction::Up => {
          vec2(0., -10.)
        },
        Direction::Down  => {
          vec2(0., 10.)
        },
        Direction::Left  => {
          vec2(-10., 0.)
        },
        Direction::Right  => {
          vec2(10., 0.)
        },
      };

      self.position = vec2(self.position.x + pos.x, self.position.y + pos.y);
    }
  }

  pub fn set_direction(&mut self, direction: Direction) {
    self.direction = direction;
  }
}
