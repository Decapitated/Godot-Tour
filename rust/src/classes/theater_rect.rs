use godot::prelude::*;
use godot::builtin::Corner;
use godot::classes::{control, notify, Control, IControl, Panel, Engine, Shader, ShaderMaterial, StyleBoxFlat};

use super::focused_node::FocusedNode;

#[derive(GodotClass)]
#[class(base = Control, tool)]
struct TheaterRect {
    base: Base<Control>,
    /// Nodes to focus on.
    #[export]
    focused_nodes: Array<Option<Gd<FocusedNode>>>,
    // Background color of unfocused area.
    #[export]
    background_color: Color,
    // Padding between unfocused and focused area.
    #[export(range=(0.0, 64.0, 1.0, or_greater, suffix="px"))]
    padding: i32,
    // Corner radius of focused area.
    #[export(range=(0.0, 64.0, 1.0, or_greater, suffix="px"))]
    corner_radius: i32,
    // Prevent mouse input outside of focused area.
    #[export]
    confine_input: bool,
    // Cached rect of focused node.
    current_rect: Rect2,
    // Cached material.
    cutout_material: Gd<ShaderMaterial>,
}

#[godot_api]
impl IControl for TheaterRect {
    fn init(base: Base<Control>) -> Self {
        // Load cutout shader and apply to material.
        let shader = load::<Shader>("res://addons/gdtour/cutout.gdshader");
        let mut material =  ShaderMaterial::new_gd();
        material.set_shader(shader);
        // Set default values.
        Self {
            base,
            focused_nodes: Array::new(),
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.9),
            padding: 16,
            corner_radius: 16,
            confine_input: true,
            current_rect: Rect2::default(),
            cutout_material: material,
        }
    }

    fn ready(&mut self) {
        let material_clone = self.cutout_material.clone();
        self.base_mut().set_material(material_clone);
    }

    fn process(&mut self, _delta: f64) {
        self.update();

        let engine = Engine::singleton();

        // Confine input to the focused control rect.
        if !engine.is_editor_hint() && self.confine_input && self.base().is_visible() {
            if let Some(viewport) = self.base().get_viewport() {
                if self.current_rect.has_point(viewport.get_mouse_position()) {
                    self.base_mut().set_mouse_filter(control::MouseFilter::IGNORE);
                } else {
                    self.base_mut().set_mouse_filter(control::MouseFilter::STOP);
                }
            }
        } else {
            self.base_mut().set_mouse_filter(control::MouseFilter::IGNORE);
        }
    }

    fn draw(&mut self) {
        let rect = self.base().get_rect();
        self.base_mut().draw_rect(rect, TheaterRect::DEFAULT_COLOR);
    }
    
    fn on_notification(&mut self, what: notify::ControlNotification) {
        match what {
            notify::ControlNotification::EDITOR_PRE_SAVE => {
                // Remove material.
                self.base_mut().set_material(None as Option<Gd<ShaderMaterial>>);
                // Reset overlays position & size.
                self.reset_overlays();
            },
            notify::ControlNotification::EDITOR_POST_SAVE => {
                let material_clone = self.cutout_material.clone();
                self.base_mut().set_material(material_clone);
                self.update();
            },
            _ => {}
        }
    }
}

#[godot_api]
impl TheaterRect {
    const DEFAULT_COLOR: Color = Color::from_rgba(1.0, 0.0, 0.0, 0.75);

    fn update(&mut self) {
        let rects = self.focused_nodes.iter_shared().map(|focused_node_result|{
            if let Some(focused_node) = focused_node_result {
                if let Some(target) = self.base().try_get_node_as::<Control>(focused_node.bind().target.clone())  {
                    return target.get_global_rect().grow(self.padding as f32);
                }
            }
            Rect2::default()
        }).collect::<Array<Rect2>>();
        self.update_shader_params(&rects);
        self.update_overlays(&rects);
        // self.update_stylebox();
    }

    fn update_shader_params(&mut self, rects: &Array<Rect2>) {
        self.cutout_material.set_shader_parameter("rects".into(), rects.to_variant());
        self.cutout_material.set_shader_parameter("corner_radius".into(), self.corner_radius.to_variant());
        self.cutout_material.set_shader_parameter("background_color".into(), self.background_color.to_variant());
    }

    fn update_overlays(&self, rects: &Array<Rect2>) {
        self.focused_nodes.iter_shared().enumerate().for_each(|(i, focused_node_result)| {
            if let Some(focused_node) = focused_node_result {
                if let Some(mut overlay) = self.base().try_get_node_as::<Panel>(focused_node.bind().overlay.clone()) {
                    let rect_result = rects.get(i);
                    if let Some(rect) = rect_result {
                        let rect = rect.grow(1.0);
                        overlay.set_position(rect.position);
                        overlay.set_size(rect.size);
                    }
                }
            }
        });
    }

    fn reset_overlays(&self) {
        self.focused_nodes.iter_shared().for_each(|focused_node_result| {
            if let Some(focused_node) = focused_node_result {
                if let Some(mut overlay) = self.base().try_get_node_as::<Panel>(focused_node.bind().overlay.clone()) {
                    overlay.set_position(Vector2::default());
                    overlay.set_size(Vector2::default());
                }
            }
        });
    }

    // fn update_stylebox(&mut self) {
    //     if let Some(overlay) = self.overlay.clone() {
    //         if let Some(stylebox) = overlay.get_theme_stylebox("panel".into()) {
    //             if let Ok(mut flat_stylebox) = stylebox.try_cast::<StyleBoxFlat>() {
    //                 flat_stylebox.set_corner_radius(Corner::BOTTOM_LEFT, self.corner_radius);
    //                 flat_stylebox.set_corner_radius(Corner::BOTTOM_RIGHT, self.corner_radius);
    //                 flat_stylebox.set_corner_radius(Corner::TOP_LEFT, self.corner_radius);
    //                 flat_stylebox.set_corner_radius(Corner::TOP_RIGHT, self.corner_radius);
    //             }
    //         }
    //     }
    // }
}
