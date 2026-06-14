use godot::{
    classes::{Area2D, IArea2D, Input},
    prelude::*,
};

use crate::ui::GameUi;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct Interactable {
    is_mouse_on_top: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Interactable {
    fn ready(&mut self) {
        let callable = &Callable::from_object_method(&self.to_gd(), "on_mouse_entered");
        self.base_mut().connect("mouse_entered", callable);
        let callable = &Callable::from_object_method(&self.to_gd(), "on_mouse_exited");
        self.base_mut().connect("mouse_exited", callable);
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        if input.is_action_just_pressed("click") && self.is_mouse_on_top {
            let mut game_ui = self.base().get_node_as::<GameUi>("%Ui");
            game_ui
                .bind_mut()
                .show_text("Yes, it's a door.\n...\nQuite sturdy too.".to_string());
        }
    }
}

#[godot_api]
impl Interactable {
    #[func]
    fn on_mouse_entered(&mut self) {
        self.is_mouse_on_top = true
    }
    #[func]
    fn on_mouse_exited(&mut self) {
        self.is_mouse_on_top = false
    }
}
