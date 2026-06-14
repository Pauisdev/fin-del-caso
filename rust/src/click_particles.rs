use godot::{
    classes::{CpuParticles2D, ICpuParticles2D, InputEvent, InputEventMouseButton},
    global::MouseButton,
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=CpuParticles2D)]
struct ClickParticle {
    base: Base<CpuParticles2D>,
}

#[godot_api]
impl ICpuParticles2D for ClickParticle {
    fn input(&mut self, event: Gd<InputEvent>) {
        let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() else {
            return;
        };
        if mouse_event.is_pressed() && mouse_event.get_button_index() == MouseButton::LEFT {
            self.base_mut().set_position(mouse_event.get_position());
            self.base_mut().set_emitting(true);
        }
    }
}
