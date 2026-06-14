use godot::prelude::*;
mod click_particles;
mod interactable;
mod text_box;
mod ui;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
