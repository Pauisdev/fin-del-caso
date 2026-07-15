use godot::classes::{Input, RichTextLabel, TextureRect};
use godot::prelude::*;

use godot::{classes::ITextureRect, register::GodotClass};

#[derive(GodotClass)]
#[class(init, base=TextureRect)]
pub struct TextBox {
    #[export]
    #[init(val = 20.0)]
    text_speed: f64,
    characters_to_show: f64,
    awaiting_release: bool,
    base: Base<TextureRect>,
}

#[godot_api]
impl ITextureRect for TextBox {
    fn ready(&mut self) {
        self.base_mut().set_visible(false);
    }

    fn process(&mut self, delta: f64) {
        self.characters_to_show += delta * self.text_speed;
        let mut label = self.base().get_node_as::<RichTextLabel>("Text");
        label.set_visible_characters(self.characters_to_show as i32);

        let input = Input::singleton();

        if input.is_action_just_released("click") {
            self.awaiting_release = false;
        }

        if input.is_action_just_pressed("click") {
            if self.awaiting_release {
                return;
            }
            let max = label.get_text().len() as i32;
            if label.get_visible_characters() < max {
                self.characters_to_show = max as f64;
                return;
            }
            self.base_mut().set_visible(false);
            godot_print!("Hecho invisible");
        }
    }
}

#[godot_api]
impl TextBox {
    #[func]
    pub fn show_text(&mut self, text: String) {
        if self.base().is_visible() {
            return;
        }
        let mut label = self.base().get_node_as::<RichTextLabel>("Text");
        label.set_visible_characters(0);
        label.set_text(&text);
        self.characters_to_show = 0.0;
        self.awaiting_release = true;
        self.base_mut().set_visible(true);
    }
}
