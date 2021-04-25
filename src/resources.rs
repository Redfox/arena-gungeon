use macroquad::prelude::{FileError, Texture2D, load_string, load_texture};
use macroquad_tiled as tiled;
use macroquad_platformer::World as CollisionWorld;

pub struct Resources {
  pub tiled_map: tiled::Map,
  pub player_texture: Texture2D,
  pub enemy_texture: Texture2D,
  pub collision_world: CollisionWorld
}

impl Resources {
  pub async fn new() -> Result<Resources, FileError> {
    let tileset = load_texture("resources/tileset.png")
      .await
      .expect("Fail to load tileset");
    tileset.set_filter(macroquad::prelude::FilterMode::Nearest);

    let tiled_map_json = load_string("resources/map.json")
      .await
      .expect("Fail to load mapjson");
    let tiled_map = tiled::load_map(
      &tiled_map_json,
      &[("tileset.png", tileset)], 
      &[]
    ).expect("Fail to load tile map");
    
    let player_texture = load_texture("resources/player-spritesheet.png")
      .await
      .expect("Fail to load playersheet");
    player_texture.set_filter(macroquad::prelude::FilterMode::Nearest);

    let enemy_texture = load_texture("resources/enemy-spritesheet.png")
      .await
      .expect("Fail to load enemysheet");
    enemy_texture.set_filter(macroquad::prelude::FilterMode::Nearest);

    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles("collision", None) {
      static_colliders.push(tile.is_some());
    }

    let mut collision_world = CollisionWorld::new();
    collision_world.add_static_tiled_layer(
      static_colliders,
      32.,
      32.,
      tiled_map.raw_tiled_map.width as _,
      1
    );

    Ok(Resources {
      tiled_map,
      player_texture,
      enemy_texture,
      collision_world
    })
  }
}
