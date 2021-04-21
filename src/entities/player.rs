use macroquad::{color, prelude::{Vec2, draw_rectangle, vec2}};

#[derive(Debug)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

pub struct Player {
  pub position: Vec2,
  pub moving: bool,
  direction: Direction,
}

impl Player {
  pub fn new() -> Self {
    Player {
      position: vec2(100., 100.),
      direction: Direction::Down,
      moving: false,
    }
  }

  pub fn draw(&self) {
    draw_rectangle(self.position.x, self.position.y, 40.0, 40.0, color::WHITE);
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
