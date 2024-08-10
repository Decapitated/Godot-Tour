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
    oneshot: bool,
}

#[godot_api]
impl IEditorPlugin for TourPlugin {
    fn init(base: Base<EditorPlugin>) -> Self {
        Self {
            base,
            theater_rect: TheaterRect::new_alloc(),
            button: Button::new_alloc(),
            oneshot: false
        }
    }

    fn enter_tree(&mut self) {
        self.setup();
    }

    fn exit_tree(&mut self) {
        self.theater_rect.queue_free();
        self.button.queue_free();
    }

    fn process(&mut self, _delta: f64) {
        if !self.oneshot {
            self.oneshot = true;
            self.test();
        }
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

    fn test(&mut self) {
        let mut run_bar_node = self.get_run_bar_control().unwrap().upcast::<Node>();
        run_bar_node.print_tree_pretty();
    }

    // fn get_base_control(&mut self) -> Option<Gd<VBoxContainer>> {
    //     let editor_interface_result = self.base_mut().get_editor_interface();
    //     if let Some(editor_interface) = editor_interface_result {
    //         if let Some(editor_interface) = editor_interface.get_base_control() {
    //             let child_result = editor_interface.get_child(0);
    //             if let Some(child) = child_result {
    //                 let vbox_result = child.try_cast::<VBoxContainer>();
    //                 if let Ok(vbox) = vbox_result {
    //                     return Some(vbox);
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }

    // fn get_title_bar_control(&mut self) -> Option<Gd<Control>> {
    //     if let Some(base) = self.get_base_control() {
    //         let titlebar_result = base.get_child(0);
    //         if let Some(titlebar) = titlebar_result {
    //             let control_result = titlebar.try_cast::<Control>();
    //             if let Ok(control) = control_result {
    //                 return Some(control);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn get_run_bar_control(&mut self) -> Option<Gd<Control>> {
    //     if let Some(titlebar) = self.get_title_bar_control() {
    //         let runbar_result = titlebar.get_child(4);
    //         if let Some(runbar) = runbar_result {
    //             let control_result = runbar.try_cast::<Control>();
    //             if let Ok(control) = control_result {
    //                 return Some(control);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn get_main_control(&mut self) -> Option<Gd<Control>> {
    //     if let Some(base) = self.get_base_control() {
    //         let main_result = base.get_child(1);
    //         if let Some(main) = main_result {
    //             let control_result = main.try_cast::<Control>();
    //             if let Ok(control) = control_result {
    //                 return Some(control);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn get_scene_tree_control(&mut self) -> Option<Gd<Control>> {
    //     if let Some(main) = self.get_main_control() {
    //         let vsplit_result = main.get_child(0);
    //         if let Some(vsplit) = vsplit_result {
    //             let control_result = vsplit.try_cast::<Control>();
    //                 if let Ok(control) = control_result {
    //                     return Some(control);
    //                 }
    //         }
    //     }
    //     None
    // }
}
