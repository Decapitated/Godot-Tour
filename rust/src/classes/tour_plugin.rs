use godot::classes::control::{LayoutPreset, MouseFilter};
use godot::classes::editor_plugin::{CustomControlContainer, DockSlot};
use godot::prelude::*;
use godot::classes::{EditorPlugin, IEditorPlugin, Control, VBoxContainer, Button};

use super::theater_rect::TheaterRect;

#[derive(GodotClass)]
#[class(tool, editor_plugin, base=EditorPlugin)]
pub struct TourPlugin {
    base: Base<EditorPlugin>,
    theater_rect: Gd<TheaterRect>,
    button: Gd<Button>,
}

#[godot_api]
impl IEditorPlugin for TourPlugin {
    fn init(base: Base<EditorPlugin>) -> Self {
        Self {
            base,
            theater_rect: TheaterRect::new_alloc(),
            button: Button::new_alloc(),
        }
    }

    fn enter_tree(&mut self) {
        self.setup();
    }

    fn exit_tree(&mut self) {
        self.theater_rect.queue_free();
        self.button.queue_free();
    }
}

#[godot_api]
impl TourPlugin {
    fn setup(&mut self) {
        // Set TheaterRect defaults.
        self.theater_rect.bind_mut().base_mut().set_anchors_preset(LayoutPreset::FULL_RECT);
        self.theater_rect.bind_mut().base_mut().set_mouse_filter(MouseFilter::IGNORE);
        self.theater_rect.bind_mut().base_mut().set_visible(false);
        
        // Set Button defaults.
        self.button.set_text("Click me!".into());
        self.button.set_custom_minimum_size(Vector2::new(80.0, 32.0));

        let button_clone = self.button.clone();
        self.base_mut().add_control_to_container(CustomControlContainer::TOOLBAR, button_clone);
    }
}
