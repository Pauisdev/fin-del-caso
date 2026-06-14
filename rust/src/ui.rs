use godot::{classes::CanvasLayer, prelude::*};

use crate::text_box::TextBox;

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct GameUi {
    base: Base<CanvasLayer>,
}

#[godot_api]
impl GameUi {
    #[func]
    pub fn show_text(&mut self, text: String) {
        let mut text_box = self.base().get_node_as::<TextBox>("TextBox");
        text_box.bind_mut().show_text(text);
    }
}
