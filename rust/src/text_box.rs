use godot::classes::{Input, RichTextLabel, TextureRect};
use godot::prelude::*;

use godot::{classes::ITextureRect, register::GodotClass};

#[derive(GodotClass)]
#[class(init, base=TextureRect)]
pub struct TextBox {
    #[export]
    #[init(val = 20.0)]
    text_speed: f64,
    text_to_show: Option<String>,
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
        let Some(text_to_show) = self.text_to_show.as_ref() else {
            return;
        };
        self.characters_to_show += delta * self.text_speed;
        let mut label = self.base().get_node_as::<RichTextLabel>("Text");
        if label.get_visible_characters() != -1 {
            label.set_visible_characters(self.characters_to_show as i32);
        }

        let input = Input::singleton();

        if input.is_action_just_released("click") {
            self.awaiting_release = false;
        }

        if input.is_action_just_pressed("click") {
            if self.awaiting_release {
                return;
            }
            if label.get_visible_characters() < text_to_show.len() as i32 {
                // TODO: this wont work anymore since im now setting it to –1
                label.set_visible_characters(-1);
                return;
            }
            self.base_mut().set_visible(false);
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
        self.text_to_show = Some(text);
        self.characters_to_show = 0.0;
        self.awaiting_release = true;
        self.base_mut().set_visible(true);
    }
}
