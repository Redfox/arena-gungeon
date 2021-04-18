pub struct MainMenuScreen;
use macroquad::{prelude::{collections::storage, vec2}, ui::{widgets, root_ui}, window::next_frame};

use crate::GuiResources;

impl MainMenuScreen {
  pub async fn render() {
    let resources = storage::get_mut::<GuiResources>();
    root_ui().push_skin(&resources.title_skin);

    let button_w = 200.0;

    if widgets::Button::new("Single Player")
       .size(vec2(button_w, 300.))
       .position(vec2(
         100., 300.
       ))
       .ui(&mut *root_ui())
    {
      println!("single player clicked");
    }

    if widgets::Button::new("Multi Player")
       .size(vec2(button_w, 300.))
       .position(vec2(
         400., 300.
       ))
       .ui(&mut *root_ui())
    {
      println!("multi player clicked");
    }

    next_frame().await;
  }
}
