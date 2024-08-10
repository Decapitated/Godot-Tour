use godot::classes::control::{CursorShape, LayoutPreset};
use godot::prelude::*;
use godot::classes::{EditorPlugin, IEditorPlugin, Button, Control, Panel};

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
    }

    #[func]
    fn get_theater_rect(&mut self) -> Gd<TheaterRect> {
        self.theater_rect.clone()
    }

    #[func]
    fn create_focused_node(&mut self, target: Option<Gd<Control>>, overlay: Option<Gd<Panel>>) -> Gd<FocusedNode> {
        let mut focused_node = FocusedNode::new_gd();
        if let Some(target) = target {
            focused_node.bind_mut().target = target.get_path();
        }
        if let Some(overlay) = overlay {
            focused_node.bind_mut().overlay = overlay.get_path();
        }
        focused_node
    }

    #[func]
    fn add_focused_node(&mut self, focused_node: Gd<FocusedNode>) {
        let mut focused_nodes = self.theater_rect.bind().get_focused_nodes();
        focused_nodes.push(Some(focused_node));
    }

    #[func]
    fn remove_focused_node(&mut self, focused_node: Gd<FocusedNode>) {
        let mut focused_nodes = self.theater_rect.bind().get_focused_nodes();
        let value = Some(focused_node);
        focused_nodes.erase(&value);
    }

    #[func]
    fn get_base_control(&mut self) -> Option<Gd<Control>> {
        let editor_interface_result = self.base_mut().get_editor_interface();
        if let Some(editor_interface) = editor_interface_result {
            return editor_interface.get_base_control();
        }
        None
    }

    #[func]
    fn get_title_bar(&mut self, base_control: Gd<Control>) -> Option<Gd<Control>> {
        let title_bar_node = base_control.get_child(0).unwrap().get_child(0).unwrap();
        if let Ok(title_bar) = title_bar_node.try_cast::<Control>() {
            return Some(title_bar);
        }
        None
    }

    #[func]
    fn get_title_bar_full(&mut self) -> Option<Gd<Control>> {
        if let Some(base_control) = self.get_base_control() {
            return self.get_title_bar(base_control);
        }
        None
    }
    
    #[func]
    fn get_run_bar(&self, title_bar: Gd<Control>) -> Option<Gd<Control>> {
        if let Ok(run_bar) = title_bar.get_child(4).unwrap().try_cast::<Control>() {
            return Some(run_bar);
        }
        None
    }
    
    #[func]
    fn get_run_bar_full(&mut self) -> Option<Gd<Control>> {
        if let Some(base_control) = self.get_base_control() {
            if let Some(title_bar) = self.get_title_bar(base_control) {
                return self.get_run_bar(title_bar.clone());
            }
        }
        None
    }
}
