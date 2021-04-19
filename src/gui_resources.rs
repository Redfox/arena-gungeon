use macroquad::{
  color::Color,
  math::RectOffset,
  texture::Image,
  ui::{
    Skin, root_ui
  }
};

pub struct GuiResources {
  pub main_menu_skin: Skin,
}

impl GuiResources {
  pub fn new() -> Self {
    let main_menu_skin = {
      let button_style = root_ui()
        .style_builder()
        .background(Image::from_file_with_format(
          include_bytes!("../resources/ui/button_background_2.png"),
          None
        ))
        .background_margin(RectOffset::new(8.0, 8.0, 12.0, 12.0))
        .margin(RectOffset::new(8.0, 8.0, 7.0, 12.0))
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
      main_menu_skin
    }
  }
}
