use godot::classes::{Input, RichTextLabel, TextureRect};
use godot::prelude::*;

use godot::{classes::ITextureRect, register::GodotClass};

#[derive(GodotClass)]
#[class(init, base=TextureRect)]
pub struct TextBox {
    #[export]
    #[init(val = 20)]
    text_speed: i32,
    text_to_show: Option<String>,
    ignored_first_click: bool,
    base: Base<TextureRect>,
}

#[godot_api]
impl ITextureRect for TextBox {
    fn ready(&mut self) {
        self.base_mut().set_visible(false);
        godot_print!("Puesto invisible");
    }

    fn process(&mut self, delta: f64) {
        godot_print!("HOLA");
        let Some(text_to_show) = self.text_to_show.as_ref() else {
            godot_print!("Falló");
            return;
        };
        godot_print!("Aqui el delta: {delta}");
        let mut label = self.base().get_node_as::<RichTextLabel>("Text");
        label.set_visible_characters(delta as i32 * self.text_speed);

        let input = Input::singleton();
        if input.is_action_just_pressed("click") {
            if !self.ignored_first_click {
                self.ignored_first_click = true;
                return;
            }
            if label.get_visible_characters() < text_to_show.len() as i32 {
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
        godot_print!("Aparecer textito");
        label.set_visible_characters(0);
        label.set_text(&text);
        self.ignored_first_click = false;
        self.base_mut().set_visible(true);
    }
}
