use godot::prelude::*;
use godot::builtin::Corner;
use godot::classes::{control, notify, Control, IControl, Panel, Engine, Shader, ShaderMaterial, StyleBoxFlat};

#[derive(GodotClass)]
#[class(base = Control, tool)]
struct TheaterRect {
    base: Base<Control>,
    // Node to focus on.
    #[export]
    focused_node: Option<Gd<Control>>,
    // Overlay node for borders and other styles/effects.
    #[export]
    #[var(get = get_overlay, set = set_overlay)]
    overlay: Option<Gd<Panel>>,
    // Background color of unfocused area.
    #[export]
    #[var(get = get_background_color, set = set_background_color)]
    background_color: Color,
    // Padding between unfocused and focused area.
    #[var(get = get_padding, set = set_padding)]
    #[export(range=(0.0, 64.0, 1.0, or_greater, suffix="px"))]
    padding: i32,
    // Corner radius of focused area.
    #[var(get = get_corner_radius, set = set_corner_radius)]
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
            focused_node: None,
            overlay: None,
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
        // Check if focused node rect has changed.
        if let Some(focused_node) = self.focused_node.clone() {
            let global_rect = focused_node.get_global_rect();
            if self.current_rect != global_rect {
                self.current_rect = global_rect;
                self.update();
            }
        }

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
                // Reset overlay position and size.
                if let Some(mut overlay) = self.overlay.clone() {
                    overlay.set_position(Vector2::default());
                    overlay.set_size(Vector2::default());
                }
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

    // region: Getters/Setters

    // region: Overlay

    #[func]
    fn get_overlay(&self) -> Option<Gd<Panel>> {
        self.overlay.clone()
    }

    #[func]
    fn set_overlay(&mut self, overlay: Option<Gd<Panel>>) {
        self.overlay = overlay;
        self.update();
    }

    // endregion

    // region: Background Color

    #[func]
    fn get_background_color(&self) -> Color {
        self.background_color
    }

    #[func]
    fn set_background_color(&mut self, background_color: Color) {
        self.background_color = background_color;
        self.update();
    }

    // endregion

    // region: Padding

    #[func]
    fn get_padding(&self) -> i32 {
        self.padding
    }

    #[func]
    fn set_padding(&mut self, padding: i32) {
        self.padding = padding;
        self.update();
    }

    // endregion

    // region: Corner Radius

    #[func]
    fn get_corner_radius(&self) -> i32 {
        self.corner_radius
    }

    #[func]
    fn set_corner_radius(&mut self, corner_radius: i32) {
        self.corner_radius = corner_radius;
        self.update();
    }

    // endregion

    // endregion

    fn update(&mut self) {
        let padded_rect = self.current_rect.grow(self.padding as f32);
        self.update_shader_params(padded_rect);
        self.update_overlay(padded_rect);
        self.update_stylebox()
    }

    fn update_shader_params(&mut self, rect: Rect2) {
        self.cutout_material.set_shader_parameter("rect_size".into(), rect.size.to_variant());
        self.cutout_material.set_shader_parameter("rect_position".into(), rect.position.to_variant());
        self.cutout_material.set_shader_parameter("corner_radius".into(), self.corner_radius.to_variant());
        self.cutout_material.set_shader_parameter("background_color".into(), self.background_color.to_variant());
    }

    fn update_overlay(&mut self, rect: Rect2) {
        if let Some(mut overlay) = self.overlay.clone() {
            let rect = rect.grow(1.0);
            overlay.set_position(rect.position);
            overlay.set_size(rect.size);
        }
    }

    fn update_stylebox(&mut self) {
        if let Some(overlay) = self.overlay.clone() {
            if let Some(stylebox) = overlay.get_theme_stylebox("panel".into()) {
                if let Ok(mut flat_stylebox) = stylebox.try_cast::<StyleBoxFlat>() {
                    flat_stylebox.set_corner_radius(Corner::BOTTOM_LEFT, self.corner_radius);
                    flat_stylebox.set_corner_radius(Corner::BOTTOM_RIGHT, self.corner_radius);
                    flat_stylebox.set_corner_radius(Corner::TOP_LEFT, self.corner_radius);
                    flat_stylebox.set_corner_radius(Corner::TOP_RIGHT, self.corner_radius);
                }
            }
        }
    }
}
