use godot::prelude::*;

use crate::ui::GameUi;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Room {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Room {
    fn ready(&mut self) {
        let mut game_ui = GameUi::new_alloc();
        game_ui.set_name("Ui");
        game_ui.set_unique_name_in_owner(true);
        self.base_mut().add_child(&game_ui);
    }
}
