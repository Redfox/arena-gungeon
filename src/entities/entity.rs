use macroquad::{color, prelude::{DrawTextureParams, Texture2D, Vec2, animation::AnimatedSprite, draw_circle, draw_texture_ex, vec2}};
use macroquad_platformer::World;

#[derive(Copy, Clone)]
pub enum Direction {
  Left = 0,
  Right = 1,
  Down = 2,
  Up = 3,
}

pub struct Entity {
  pub position: Vec2,
  pub moving: bool,
  pub sprite: AnimatedSprite,
  pub direction: Direction,
  pub speed: u8,
}

impl Entity {
  pub fn draw(&mut self, texture: Texture2D) {
    let debug = false;
    if debug {
      // collid top left
      draw_circle(self.position.x + 7., self.position.y + 7., 2., color::BLACK);

      // collid top right
      draw_circle(self.position.x + 32. - 7., self.position.y + 7., 2., color::BLACK);

      // collid down left
      draw_circle(self.position.x + 7., self.position.y + 32. + 5., 2., color::BLACK);

      // collid down right
      draw_circle(self.position.x + 32. - 7., self.position.y + 32. + 5., 2., color::BLACK);
    }

    draw_texture_ex(
      texture, 
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

    let point_left_top = vec2(self.position.x + 7., self.position.y + 7.);
    let point_right_top = vec2(self.position.x + 32. - 7., self.position.y + 7.);
    let point_left_bottom = vec2(self.position.x + 7., self.position.y + 32. + 5.);
    let point_right_bottom = vec2(self.position.x + 32. - 7., self.position.y + 32. + 5.);

    if self.moving {
      let collide_top = 
        (collision_world.solid_at(point_left_top) && !collision_world.solid_at(point_left_bottom)) || 
        (collision_world.solid_at(point_right_top) && !collision_world.solid_at(point_right_bottom));

      let collide_bottom = 
        (collision_world.solid_at(point_left_bottom) && !collision_world.solid_at(point_left_top)) || 
        (collision_world.solid_at(point_right_bottom) && !collision_world.solid_at(point_right_top));

      let collide_left = 
        (collision_world.solid_at(point_left_bottom) && !collision_world.solid_at(point_right_bottom)) || 
        (collision_world.solid_at(point_left_top) && !collision_world.solid_at(point_right_top));

      let collide_right = 
        (collision_world.solid_at(point_right_top) && !collision_world.solid_at(point_left_top)) ||
        (collision_world.solid_at(point_right_bottom) && !collision_world.solid_at(point_left_bottom));

      let pos = match self.direction {
        Direction::Up => {
          if collide_top  {
            vec2(0., 0.)
          } else {
            vec2(0., -(self.speed as f32))
          }
        },
        Direction::Down  => {
          if collide_bottom {
            vec2(0., 0.)
          } else {
            vec2(0., self.speed as f32)
          }
        },
        Direction::Left  => {
          if collide_left {
            vec2(0., 0.)
          } else {
            vec2(-(self.speed as f32), 0.)
          }
        },
        Direction::Right  => {
          if collide_right {
            vec2(0., 0.)
          } else {
            vec2(self.speed as f32, 0.)
          }
        },
      };

      self.position = vec2(self.position.x + pos.x, self.position.y + pos.y);
    }
  }
}