use godot::classes::control::{CursorShape, LayoutPreset};
use godot::prelude::*;
use godot::classes::{Object, IObject, Control, Panel};

use super::focused_node::FocusedNode;
use super::theater_rect::TheaterRect;
use super::tour_plugin::TourPlugin;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct TourSingleton {
    base: Base<Object>,
    #[export]
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
    fn clear_focused_nodes(&mut self) {
        let mut focused_nodes = self.theater_rect.bind().get_focused_nodes();
        focused_nodes.clear();
    }

    #[func]
    pub fn get_base_control(&self) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_base_control();
        }
        None
    }

    #[func]
    pub fn get_title_bar(&self, base_control: Gd<Control>) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_title_bar(base_control);
        }
        None
    }

    #[func]
    pub fn get_title_bar_full(&self) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_title_bar_full();
        }
        None
    }

    #[func]
    pub fn get_main(&mut self, base_control: Gd<Control>) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_main(base_control);
        }
        None
    }

    #[func]
    pub fn get_main_full(&mut self) -> Option<Gd<Control>> {
        if let Some(mut tour_plugin) = self.tour_plugin.clone() {
            return tour_plugin.bind_mut().get_main_full();
        }
        None
    }
}
