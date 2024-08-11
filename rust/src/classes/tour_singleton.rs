use godot::classes::control::{CursorShape, LayoutPreset};
use godot::prelude::*;
use godot::classes::{Object, IObject, Control, Panel};

use super::focused_node::FocusedNode;
use super::theater_rect::TheaterRect;
use super::tour_plugin::TourPlugin;

#[derive(GodotClass)]
#[class(base=Object, rename=Tour)]
/// Singleton for controlling the an editor tour.
pub struct TourSingleton {
    base: Base<Object>,
    #[var]
    pub theater_rect: Gd<TheaterRect>,
    pub tour_plugin: Option<Gd<TourPlugin>>,
}

#[godot_api]
impl IObject for TourSingleton {
    fn init(base: Base<Object>) -> Self {
        let mut theater_rect = TheaterRect::new_alloc();
        theater_rect.bind_mut().base_mut().set_anchors_preset(LayoutPreset::FULL_RECT);
        theater_rect.bind_mut().base_mut().set_default_cursor_shape(CursorShape::FORBIDDEN);
        Self {
            base,
            theater_rect,
            tour_plugin: None
        }
    }
}

#[godot_api]
impl TourSingleton {
    pub fn find_child(mut node: Gd<Node>, pattern: &str, recursive: bool, owned: bool) -> Option<Gd<Node>> {
        let find_child_args = array![pattern.to_variant(), recursive.to_variant(), owned.to_variant()];
        let result = node.callv("find_child".into(), find_child_args);
        let node_result = result.try_to();
        if let Ok(node) = node_result {
            return Some(node);
        }
        None
    }

    pub fn find_children(mut node: Gd<Node>, pattern: &str, node_type: &str, recursive: bool, owned: bool) -> Option<Array<Gd<Node>>> {
        let find_children_args = array![pattern.to_variant(), node_type.to_variant(), recursive.to_variant(), owned.to_variant()];
        let result = node.callv("find_children".into(), find_children_args);
        let node_result = result.try_to();
        if let Ok(node) = node_result {
            return Some(node);
        }
        None
    }

    //#region Focused Nodes

    /// Helper function for creating a focused node resource.
    #[func]
    fn create_focused_node(target: Option<Gd<Control>>, overlay: Option<Gd<Panel>>) -> Gd<FocusedNode> {
        let mut focused_node = FocusedNode::new_gd();
        if let Some(target) = target {
            focused_node.bind_mut().target = target.get_path();
        }
        if let Some(overlay) = overlay {
            focused_node.bind_mut().overlay = overlay.get_path();
        }
        focused_node
    }

    /// Helper function for adding a focused node to the theater_rect.
    #[func]
    fn add_focused_node(&mut self, focused_node: Gd<FocusedNode>) {
        let mut focused_nodes = self.theater_rect.bind().get_focused_nodes();
        focused_nodes.push(Some(focused_node));
    }

    /// Helper function for removing a focused node from the theater_rect.
    #[func]
    fn remove_focused_node(&mut self, focused_node: Gd<FocusedNode>) {
        let mut focused_nodes = self.theater_rect.bind().get_focused_nodes();
        let value = Some(focused_node);
        focused_nodes.erase(&value);
    }

    /// Helper function for removing all focused nodes from the theater_rect.
    #[func]
    fn clear_focused_nodes(&mut self) {
        let mut focused_nodes = self.theater_rect.bind().get_focused_nodes();
        focused_nodes.clear();
    }

    //#endregion

    //#region TourPlugin function binds.

    /// Get base control of editor window.
    #[func]
    pub fn get_base_control(&self) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_base_control();
        }
        None
    }

    /// Get title bar control of editor window.
    #[func]
    pub fn get_title_bar(&self, base_control: Gd<Control>) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_title_bar(base_control);
        }
        None
    }

    /// Get title bar control of editor window.
    #[func]
    pub fn get_title_bar_full(&self) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_title_bar_full();
        }
        None
    }

    //#endregion

    /// Get run bar control in title bar.
    #[func]
    pub fn get_run_bar(&self, title_bar: Gd<Control>) -> Option<Gd<Control>> {
        if let Some(nodes) = TourSingleton::find_children(title_bar.upcast::<Node>(), "*EditorRunBar*", "EditorRunBar", false, false) {
            if let Some(node) = nodes.get(0) {
                if let Ok(run_bar) = node.try_cast::<Control>() {
                    return Some(run_bar);
                }
            }
        }
        None
    }

    /// Get run bar control in title bar.
    #[func]
    pub fn get_run_bar_full(&self) -> Option<Gd<Control>> {
        if let Some(title_bar) = self.get_title_bar_full() {
            if let Some(nodes) = TourSingleton::find_children(title_bar.upcast::<Node>(), "*EditorRunBar*", "EditorRunBar", false, false) {
                if let Some(node) = nodes.get(0) {
                    if let Ok(run_bar) = node.try_cast::<Control>() {
                        return Some(run_bar);
                    }
                }
            }
        }
        None
    }
    
    /// Get main control of editor window. This control holds everything below the title bar.
    #[func]
    pub fn get_main(&self, base_control: Gd<Control>) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_main(base_control);
        }
        None
    }

    /// Get main control of editor window. This control holds everything below the title bar.
    #[func]
    pub fn get_main_full(&self) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_main_full();
        }
        None
    }

    #[func]
    pub fn get_scene_tree_dock(&self, main: Gd<Control>) -> Option<Gd<Control>> {
        let nodes_result = TourSingleton::find_children(main.upcast(), "Scene", "SceneTreeDock", true, false);
        if let Some(nodes) = nodes_result {
            let node_result = nodes.get(0); 
            if let Some(node) = node_result {
                if let Ok(control) = node.try_cast::<Control>() {
                    return Some(control);
                }
            }
        }
        None
    }
}
