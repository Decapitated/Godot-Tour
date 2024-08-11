use godot::prelude::*;
use godot::classes::{Button, Control, EditorPlugin, IEditorPlugin, Tree, TreeItem, Label};

use super::tour_singleton::TourSingleton;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
pub struct TourPlugin {
    base: Base<EditorPlugin>,
    tree: Option<Gd<Tree>>,
}

#[godot_api]
impl IEditorPlugin for TourPlugin {
    fn enter_tree(&mut self) {
        if let Some(mut base_control) = self.get_base_control() {
            let mut tour_singleton = TourPlugin::get_tour_singleton();
            // Make plugin available to singleton.
            tour_singleton.bind_mut().tour_plugin = Some(self.to_gd());
            // Add TheaterRect to base_control.
            base_control.add_child(tour_singleton.bind().theater_rect.clone());

            self.tree = TourPlugin::create_tree(Some(base_control.clone()));
            let tree_clone = self.tree.clone();
            self.base_mut().add_control_to_bottom_panel(tree_clone, "Editor Tree".into());
            
            if let Some(mut title_bar) = self.get_title_bar(base_control) {
                let mut update_tree_button = Button::new_alloc();
                update_tree_button.set_text("Update Tree".into());
                update_tree_button.connect("pressed".into(), self.base().callable("update_tree"));

                title_bar.add_child(update_tree_button.clone());
                title_bar.move_child(update_tree_button, 4);
            }
        }
    }
}

#[godot_api]
impl TourPlugin {
    #[func]
    fn update_tree(&mut self) {
        if let Some(base_control) = self.get_base_control() {
            let tree_clone = self.tree.clone();
            self.base_mut().remove_control_from_bottom_panel(tree_clone);
            self.tree = TourPlugin::create_tree(Some(base_control.clone()));
            let tree_clone = self.tree.clone();
            self.base_mut().add_control_to_bottom_panel(tree_clone, "Editor Tree".into());
        }
    }

    fn create_tree(control: Option<Gd<Control>>) -> Option<Gd<Tree>> {
        if let Some(control) = control {
            let mut tree = Tree::new_alloc();
            let root = tree.create_item();
            if let Some(mut root) = root {
                root.set_text(0, format!("{:?} -> {:?} = {:?}", control.get_name(), control.get_class(), control).into());
                root.set_metadata(0, control.to_variant());
                control.get_children().iter_shared().for_each(|child| {
                    if let Ok(child_control) = child.try_cast::<Control>() {
                        TourPlugin::create_tree_item(&mut root, &child_control)
                    }
                });
            }
            return Some(tree);
        }
        None
    }

    fn create_tree_item(parent: &mut Gd<TreeItem>, control: &Gd<Control>) {
        let node_item = parent.create_child();
        if let Some(mut node_item) = node_item {
            let visibility_string = if control.is_visible() { "Visible" } else { "Hidden" };
            if let Ok(label) = control.clone().try_cast::<Label>() {
                node_item.set_text(0, format!("({}) {} -> {} = {}", visibility_string, control.get_name(), control.get_class(), label.get_text()).into());
            } else {
                node_item.set_text(0, format!("({}) {} -> {}", visibility_string, control.get_name(), control.get_class()).into());
            }
            node_item.set_metadata(0, control.to_variant());
            if control.is_visible() {
                control.get_children().iter_shared().for_each(|child| {
                    if let Ok(child_control) = child.try_cast::<Control>() {
                        TourPlugin::create_tree_item(&mut node_item, &child_control)
                    }
                });
            }
        }
    }

    fn get_tour_singleton() -> Gd<TourSingleton> {
        godot::classes::Engine::singleton().get_singleton(StringName::from("Tour")).unwrap().cast::<TourSingleton>()
    }

    pub fn get_base_control(&mut self) -> Option<Gd<Control>> {
        let editor_interface_result = self.base_mut().get_editor_interface();
        if let Some(editor_interface) = editor_interface_result {
            return editor_interface.get_base_control();
        }
        None
    }

    pub fn get_title_bar(&mut self, base_control: Gd<Control>) -> Option<Gd<Control>> {
        let title_bar_node = base_control.get_child(0).unwrap().get_child(0).unwrap();
        if let Ok(title_bar) = title_bar_node.try_cast::<Control>() {
            return Some(title_bar);
        }
        None
    }

    pub fn get_title_bar_full(&mut self) -> Option<Gd<Control>> {
        if let Some(base_control) = self.get_base_control() {
            return self.get_title_bar(base_control);
        }
        None
    }

    pub fn get_main(&mut self, base_control: Gd<Control>) -> Option<Gd<Control>> {
        let main_node = base_control.get_child(0).unwrap().get_child(1).unwrap();
        if let Ok(main) = main_node.try_cast::<Control>() {
            return Some(main);
        }
        None
    }

    pub fn get_main_full(&mut self) -> Option<Gd<Control>> {
        if let Some(base_control) = self.get_base_control() {
            return self.get_main(base_control);
        }
        None
    }
}
