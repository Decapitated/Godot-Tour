use godot::classes::control::{CursorShape, LayoutPreset, MouseFilter};
use godot::classes::editor_plugin::CustomControlContainer;
use godot::prelude::*;
use godot::classes::{EditorPlugin, IEditorPlugin, Button};

use super::focused_node::FocusedNode;
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
        self.theater_rect.bind_mut().base_mut().set_default_cursor_shape(CursorShape::FORBIDDEN);

        let editor_interface_result = self.base_mut().get_editor_interface();
        if let Some(editor_interface) = editor_interface_result {
            if let Some(mut base_control) = editor_interface.get_base_control() {
                let theater_rect_clone = self.theater_rect.clone();
                base_control.add_child(theater_rect_clone);
            }
        }
        
        let editor_plugin = self.base().clone();
        // Set Button defaults.
        self.button.set_text("Click me!".into());
        // Connect to pressed signal.
        self.button.connect("pressed".into(), editor_plugin.callable("on_pressed"));

        let button_clone = self.button.clone();
        self.base_mut().add_control_to_container(CustomControlContainer::TOOLBAR, button_clone);

        let button_parent = self.button.get_parent().unwrap();

        let mut focused_nodes = self.theater_rect.bind().get_focused_nodes();
        let mut focused_node = FocusedNode::new_gd();
        focused_node.bind_mut().target = button_parent.get_path();
        focused_nodes.push(Some(focused_node));
    }

    #[func]
    fn on_pressed(&self) {
        godot_print!("Button pressed!");
    }
}
