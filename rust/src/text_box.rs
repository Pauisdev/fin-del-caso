use godot::classes::{Input, RichTextLabel, TextureRect};
use godot::prelude::*;

use godot::{classes::ITextureRect, register::GodotClass};

#[derive(GodotClass)]
#[class(init, base=TextureRect)]
struct TextBox {
    #[export]
    #[init(val = 20.0)]
    text_speed: f64,

    #[init(val = None)]
    text_to_show: Option<String>,
    #[init(val = 0.0)]
    letters_to_show: f64,
    #[init(val = false)]
    ignored_first_click: bool,
    base: Base<TextureRect>,
}

#[godot_api]
impl ITextureRect for TextBox {
    fn process(&mut self, delta: f64) {
        let Some(text_to_show) = self.text_to_show.as_ref() else {
            return;
        };

        self.letters_to_show += delta * self.text_speed;
        let mut label = self.base().get_node_as::<RichTextLabel>("RichTextLabel");
        let text_to_show: String = text_to_show
            .chars()
            .take(self.letters_to_show.round() as usize)
            .collect();
        label.set_text(text_to_show.as_str());

        let input = Input::singleton();
        if input.is_action_just_pressed("click") {
            if !self.ignored_first_click {
                self.ignored_first_click = true;
                return;
            }
            if self.letters_to_show < text_to_show.len() as f64 {
                self.letters_to_show = text_to_show.len() as f64;
                return;
            }
            self.base_mut().set_visible(false);
        }
    }
}

#[godot_api]
impl TextBox {
    #[func]
    fn show_text(&mut self, text: String) {
        if self.base().is_visible() {
            return;
        }
        self.text_to_show = Some(text);
        self.letters_to_show = 0.0;
        self.ignored_first_click = false;
        self.base_mut().set_visible(true);
    }
}
