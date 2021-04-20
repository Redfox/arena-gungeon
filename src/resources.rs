use macroquad::prelude::{FileError, load_string, load_texture};
use macroquad_tiled as tiled;

pub struct Resources {
  pub tiled_map: tiled::Map,
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
    
    Ok(Resources {
      tiled_map
    })
  }
}
