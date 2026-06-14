use godot::prelude::*;
mod door;
mod text_box;
mod ui;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
