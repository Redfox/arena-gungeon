use macroquad::{
  color,
  math::{Rect},
  prelude::{
    DrawTextureParams,
    Texture2D,
    Vec2,
    draw_texture_ex,
    load_texture,
    vec2
  }
};

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
  direction: Direction,
  texture: Texture2D
}

impl Player {
  pub async fn new() -> Self {
    let texture = load_texture("resources/player-sheet.png").await.expect("msg");
    texture.set_filter(macroquad::prelude::FilterMode::Nearest);

    Player {
      position: vec2(100., 100.),
      direction: Direction::Down,
      moving: false,
      texture
    }
  }

  pub fn draw(&self) {
    // draw_rectangle(self.position.x, self.position.y, 40.0, 40.0, color::WHITE);
    
    let tile_size = 16;

    let source = Rect::new(
      (tile_size * (self.direction as u8)) as f32,
      (tile_size * 0) as f32,
      tile_size as f32,
      tile_size as f32,
    );

    draw_texture_ex(
      self.texture, 
      self.position.x, 
      self.position.y, 
      color::WHITE,
      DrawTextureParams {
        source: Some(source),
        dest_size: Some(vec2((tile_size as f32) * 3., (tile_size  as f32)* 3.)),
        ..Default::default()
      }
    )
  }

  pub fn update(&mut self) {
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
