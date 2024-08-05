use godot::prelude::*;
use godot::classes::{control, notify, ColorRect, Control, IColorRect, Shader, ShaderMaterial, Engine, Theme};

#[derive(GodotClass)]
#[class(base = ColorRect, tool)]
struct TheaterRect {
    base: Base<ColorRect>,
    // Node to focus on.
    #[export]
    focused_node: Option<Gd<Control>>,
    // Overlay node for borders and other styles/effects.
    #[export]
    #[var(get = get_overlay, set = set_overlay)]
    overlay: Option<Gd<Control>>,
    // Background color of unfocused area.
    #[export]
    #[var(get = get_background_color, set = set_background_color)]
    background_color: Color,
    // Padding between unfocused and focused area.
    #[export]
    #[var(get = get_padding, set = set_padding)]
    padding: i32,
    // Corner radius of focused area.
    #[export]
    #[var(get = get_corner_radius, set = set_corner_radius)]
    corner_radius: i32,
    // Prevent mouse input outside of focused area.
    #[export]
    confine_input: bool,
    // Cached rect of focused node.
    current_rect: Rect2,
    // Cached material.
    cutout_material: Gd<ShaderMaterial>,
    theme: Gd<Theme>,
}

#[godot_api]
impl IColorRect for TheaterRect {
    fn init(base: Base<ColorRect>) -> Self {
        Self {
            base,
            focused_node: None,
            overlay: None,
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.9),
            padding: 16,
            corner_radius: 0,
            confine_input: true,
            current_rect: Rect2::default(),
            cutout_material: ShaderMaterial::new_gd(),
            theme: load::<Theme>("res://addons/gdtour/TheaterRect.theme"),
        }
    }

    fn ready(&mut self) {
        // Load cutout shader and apply to TheaterRect material.
        let shader = load::<Shader>("res://addons/gdtour/cutout.gdshader");
        self.cutout_material.set_shader(shader);
        let material_clone = self.cutout_material.clone();
        self.base_mut().set_material(material_clone);

        let theme_clone = self.theme.clone();
        self.base_mut().set_theme(theme_clone);
    }

    fn process(&mut self, _delta: f64) {
        if let Some(focused_node) = self.focused_node.clone() {
            let global_rect = focused_node.get_global_rect();
            if self.current_rect != global_rect {
                self.current_rect = global_rect;
                self.update();
            }
        }

        let engine = Engine::singleton();

        // Confine input to the focused node.
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

    fn on_notification(&mut self, what: notify::ControlNotification) {
        match what {
            notify::ControlNotification::EDITOR_PRE_SAVE => {
                // Remove material.
                self.base_mut().set_material(None as Option<Gd<ShaderMaterial>>);
                // Remove theme.
                self.base_mut().set_theme(None as Option<Gd<Theme>>);
                // Reset overlay position and size.
                if let Some(mut overlay) = self.overlay.clone() {
                    overlay.set_position(Vector2::default());
                    overlay.set_size(Vector2::default());
                }
            },
            notify::ControlNotification::EDITOR_POST_SAVE => {
                let material_clone = self.cutout_material.clone();
                self.base_mut().set_material(material_clone);
                let theme_clone = self.theme.clone();
                self.base_mut().set_theme(theme_clone);
                self.update();
            },
            _ => {}
        }
    }
}

#[godot_api]
impl TheaterRect {
    // region: Getters/Setters

    // region: Overlay

    #[func]
    fn get_overlay(&self) -> Option<Gd<Control>> {
        self.overlay.clone()
    }

    #[func]
    fn set_overlay(&mut self, overlay: Option<Gd<Control>>) {
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
}
