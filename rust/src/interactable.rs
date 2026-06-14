use godot::{
    classes::{Area2D, IArea2D, Input},
    prelude::*,
};

use crate::ui::GameUi;

#[derive(GodotConvert, Var, Export, Default, Clone, Copy)]
#[godot(via = GString)]
enum Room {
    #[default]
    Nowhere,
    Main,
    LeftRoom,
}

impl Room {
    fn to_scene_name(self) -> Option<String> {
        match self {
            Self::Nowhere => None,
            Self::LeftRoom => Some(String::from("left_room")),
            Self::Main => Some(String::from("main")),
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct Interactable {
    #[export(multiline)]
    message: GString,
    #[export]
    leads_to: Room,
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
            game_ui.bind_mut().show_text(self.message.to_string());
            if let Some(scene_name) = self.leads_to.to_scene_name() {
                let scene_path = format!("res://scenes/{scene_name}.tscn");
                self.base().get_tree().change_scene_to_file(&scene_path);
            }
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
