use godot::prelude::*;

mod classes;

struct GodotTour;

#[gdextension]
unsafe impl ExtensionLibrary for GodotTour {}
