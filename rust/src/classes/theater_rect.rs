use godot::prelude::*;
use godot::builtin::Corner;
use godot::classes::{control, notify, Control, Engine, IControl, Panel, Shader, ShaderMaterial, StyleBoxFlat, StyleBoxTexture};

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
    // Prevent mouse input outside of focused area.
    #[export]
    confine_input: bool,
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
            confine_input: true,
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
            // if let Some(viewport) = self.base().get_viewport() {
            //     if self.current_rect.has_point(viewport.get_mouse_position()) {
            //         self.base_mut().set_mouse_filter(control::MouseFilter::IGNORE);
            //     } else {
            //         self.base_mut().set_mouse_filter(control::MouseFilter::STOP);
            //     }
            // }
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
            },
            _ => {}
        }
    }
}

#[godot_api]
impl TheaterRect {
    const DEFAULT_COLOR: Color = Color::from_rgba(1.0, 0.0, 0.0, 0.75);

    fn update(&mut self) {
        self.update_shader_params();
        self.update_overlays();
    }

    fn update_shader_params(&mut self) {
        let rects = self.get_rects();
        let corners = self.get_corners();
        self.cutout_material.set_shader_parameter("rects".into(), rects.to_variant());
        self.cutout_material.set_shader_parameter("corners".into(), corners.to_variant());
        self.cutout_material.set_shader_parameter("background_color".into(), self.background_color.to_variant());
    }

    fn update_overlays(&self) {
        self.focused_nodes.iter_shared().for_each(|focused_node_result| {
            if let Some(focused_node) = focused_node_result {
                let target_nodepath = focused_node.bind().target.clone();
                if let Some(target) = self.base().try_get_node_as::<Control>(target_nodepath) {
                    let overlay_nodepath = focused_node.bind().overlay.clone();
                    if let Some(mut overlay) = self.base().try_get_node_as::<Panel>(overlay_nodepath) {
                        let rect = target.get_global_rect().grow(1.0);
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
                let overlay_nodepath = focused_node.bind().overlay.clone();
                if let Some(mut overlay) = self.base().try_get_node_as::<Panel>(overlay_nodepath) {
                    overlay.set_position(Vector2::default());
                    overlay.set_size(Vector2::default());
                }
            }
        });
    }

    fn get_rects(&self) -> Array<Rect2> {
        self.focused_nodes.iter_shared().map(|focused_node_result|{
            if let Some(focused_node) = focused_node_result {
                if let Some(target) = self.base().try_get_node_as::<Control>(focused_node.bind().target.clone()) {
                    let target_rect = target.get_global_rect();
                    let overlay_nodepath = focused_node.bind().overlay.clone();
                    if let Some(overlay) = self.base().try_get_node_as::<Panel>(overlay_nodepath) {
                        if let Some(stylebox) = overlay.get_theme_stylebox("panel".into()) {
                            if let Ok(flat_stylebox) = stylebox.clone().try_cast::<StyleBoxFlat>() {
                                return target_rect.grow_individual(
                                    flat_stylebox.get_expand_margin(Side::LEFT),
                                    flat_stylebox.get_expand_margin(Side::TOP),
                                    flat_stylebox.get_expand_margin(Side::RIGHT),
                                    flat_stylebox.get_expand_margin(Side::BOTTOM),
                                );
                            } else if let Ok(texture_stylebox) = stylebox.try_cast::<StyleBoxTexture>() {
                                return target_rect.grow_individual(
                                    texture_stylebox.get_expand_margin(Side::LEFT),
                                    texture_stylebox.get_expand_margin(Side::TOP),
                                    texture_stylebox.get_expand_margin(Side::RIGHT),
                                    texture_stylebox.get_expand_margin(Side::BOTTOM),
                                );
                            }
                        }
                    }
                    return target_rect;
                }
            }
            Rect2::default()
        }).collect()
    }

    fn get_corners(&self) -> Array<f32> {
        self.focused_nodes.iter_shared().map(|focused_node_result|{
            if let Some(focused_node) = focused_node_result {
                let overlay_nodepath = focused_node.bind().overlay.clone();
                if let Some(overlay) = self.base().try_get_node_as::<Panel>(overlay_nodepath) {
                    if let Some(stylebox) = overlay.get_theme_stylebox("panel".into()) {
                        if let Ok(stylebox_flat) = stylebox.try_cast::<StyleBoxFlat>() {
                            return stylebox_flat.get_corner_radius(Corner::TOP_LEFT) as f32;
                        }
                    }
                }
            }
            0.0
        }).collect()
    }
}
