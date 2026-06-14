use godot::prelude::*;
mod text_box;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
