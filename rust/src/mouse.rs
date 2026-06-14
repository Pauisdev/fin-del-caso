use godot::classes::input::MouseMode;
use godot::classes::{ISprite2D, Input, Sprite2D};
use godot::prelude::*;
use godot::register::GodotClass;

#[derive(GodotClass)]
#[class(init, base=Sprite2D)]
struct Mouse {
    #[export]
    #[init(val = 16)]
    grid_size: i32,
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Mouse {
    fn ready(&mut self) {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::HIDDEN);
    }

    fn process(&mut self, _delta: f64) {
        let viewport = self.base().get_viewport().unwrap();
        let mouse_pos = viewport.get_mouse_position();
        let final_pos = Vector2 {
            x: (mouse_pos.x / self.grid_size as f32).round() * self.grid_size as f32,
            y: (mouse_pos.y / self.grid_size as f32).round() * self.grid_size as f32,
        };

        self.base_mut().set_position(final_pos);
    }
}
