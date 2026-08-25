use godot::{
    classes::{CanvasLayer, ICanvasLayer, TextureRect},
    prelude::*,
};

use crate::text_box::TextBox;

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct GameUi {
    base: Base<CanvasLayer>,
    #[init(node = "Character")]
    pub character: OnReady<Gd<TextureRect>>,
    #[init(node = "Shadow")]
    pub shadow: OnReady<Gd<TextureRect>>,
    #[init(node = "Badge")]
    pub badge: OnReady<Gd<TextureRect>>,
    #[init(node = "TextBox")]
    pub text_box: OnReady<Gd<TextBox>>,
}

#[godot_api]
impl ICanvasLayer for GameUi {
    fn ready(&mut self) {
        self.character.set_visible(false);
        self.shadow.set_visible(false);
        self.badge.set_visible(false);
    }
}

#[godot_api]
impl GameUi {
    pub fn show_text(&self, text: String) {
        let mut text_box = self.base().get_node_as::<TextBox>("TextBox");
        text_box.bind_mut().show_text(text);
    }
}
