use godot::prelude::*;
use godot::classes::{EditorPlugin, IEditorPlugin, Control};

use super::tour_singleton::TourSingleton;

#[derive(GodotClass)]
#[class(tool, editor_plugin, base=EditorPlugin)]
pub struct TourPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for TourPlugin {
    fn init(base: Base<EditorPlugin>) -> Self {
        Self {
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.setup();
    }

    fn exit_tree(&mut self) {}
}

#[godot_api]
impl TourPlugin {
    fn setup(&mut self) {
        if let Some(mut base_control) = self.get_base_control() {
            let mut tour_singleton = TourPlugin::get_tour_singleton();
            // Make plugin available to singleton.
            tour_singleton.bind_mut().tour_plugin = Some(self.to_gd());
            // Add TheaterRect to base_control.
            base_control.add_child(tour_singleton.bind().theater_rect.clone());
        }
    }

    fn get_tour_singleton() -> Gd<TourSingleton> {
        godot::classes::Engine::singleton().get_singleton(StringName::from("TourSingleton")).unwrap().cast::<TourSingleton>()
    }

    #[func]
    pub fn get_base_control(&mut self) -> Option<Gd<Control>> {
        let editor_interface_result = self.base_mut().get_editor_interface();
        if let Some(editor_interface) = editor_interface_result {
            return editor_interface.get_base_control();
        }
        None
    }

    #[func]
    pub fn get_title_bar(&mut self, base_control: Gd<Control>) -> Option<Gd<Control>> {
        let title_bar_node = base_control.get_child(0).unwrap().get_child(0).unwrap();
        if let Ok(title_bar) = title_bar_node.try_cast::<Control>() {
            return Some(title_bar);
        }
        None
    }

    #[func]
    pub fn get_title_bar_full(&mut self) -> Option<Gd<Control>> {
        if let Some(base_control) = self.get_base_control() {
            return self.get_title_bar(base_control);
        }
        None
    }
    
    #[func]
    pub fn get_run_bar(&self, title_bar: Gd<Control>) -> Option<Gd<Control>> {
        if let Ok(run_bar) = title_bar.get_child(4).unwrap().try_cast::<Control>() {
            return Some(run_bar);
        }
        None
    }
    
    #[func]
    pub fn get_run_bar_full(&mut self) -> Option<Gd<Control>> {
        if let Some(base_control) = self.get_base_control() {
            if let Some(title_bar) = self.get_title_bar(base_control) {
                return self.get_run_bar(title_bar.clone());
            }
        }
        None
    }
}
