use scene_manager::SceneManager;
use constants::{WINDOW_HEIGHT, WINDOW_WIDHT, WINDOW_TITLE};
use macroquad::{color::Color, math::RectOffset, prelude::{Conf, collections::storage}, texture::Image, ui::{Skin, root_ui}};

mod scenes;
mod scene_manager;
mod constants;

fn window_conf() -> Conf {
  Conf {
    window_title: String::from(WINDOW_TITLE),
    fullscreen: false,
    window_height: WINDOW_HEIGHT,
    window_width: WINDOW_WIDHT,
    window_resizable: false,
    ..Default::default()
  }
}

pub struct GuiResources {
  title_skin: Skin,
}

impl GuiResources {
  fn new() -> Self {
    let title_skin = {
      let button_style = root_ui()
        .style_builder()
        .background(Image::from_file_with_format(
          include_bytes!("../resources/ui/button_background_2.png"),
          None
        ))
        .background_margin(RectOffset::new(8.0, 8.0, 12.0, 12.0))
        .margin(RectOffset::new(8.0, 8.0, 110.0, 12.0))
        // .background_hovered(background_hovered)
        .text_color(Color::from_rgba(200, 200, 160, 255))
        .font_size(34)
        .build();

      Skin {
        button_style,
        ..root_ui().default_skin()
      }
    };

    GuiResources {
      title_skin
    }
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let gui_resources = GuiResources::new();
  storage::store(gui_resources);

  let scene_manager = SceneManager::new();

  loop {
    scene_manager.render_scene().await;
  }
}
