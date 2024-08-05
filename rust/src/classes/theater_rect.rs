use godot::prelude::*;
use godot::classes::{ColorRect, IColorRect, Control, InputEvent, InputEventMouse, ShaderMaterial, Shader, notify};

#[derive(GodotClass)]
#[class(base = ColorRect, tool)]
struct TheaterRect {
    base: Base<ColorRect>,
    // Node to focus on.
    #[export]
    focused_node: Option<Gd<Control>>,
    // Background color of unfocused area.
    #[export]
    #[var(get = get_background_color, set = set_background_color)]
    background_color: Color,
    // Border color of unfocused area.
    #[export]
    #[var(get = get_border_color, set = set_border_color)]
    border_color: Color,
    // Border width of unfocused area.
    #[export(range=(0.0, 32.0, 1.0, or_greater, suffix = "px"))]
    #[var(get = get_border_width, set = set_border_width)]
    border_width: i32,
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
}

#[godot_api]
impl IColorRect for TheaterRect {
    fn init(base: Base<ColorRect>) -> Self {
        Self {
            base,
            focused_node: None,
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.9),
            border_color: Color::from_rgba(1.0, 0.0, 0.0, 1.0),
            border_width: 0,
            padding: 16,
            corner_radius: 0,
            confine_input: true,
            current_rect: Rect2::default(),
            cutout_material: ShaderMaterial::new_gd(),
        }
    }

    fn ready(&mut self) {
        let shader = load::<Shader>("res://addons/gdtour/cutout.gdshader");
        self.cutout_material.set_shader(shader);
        let material_clone = self.cutout_material.clone();
        self.base_mut().set_material(material_clone);
    }

    fn process(&mut self, _delta: f64) {
        if let Some(focused_node) = self.focused_node.clone() {
            let global_rect = focused_node.get_global_rect();
            if self.current_rect != global_rect {
                self.current_rect = global_rect;
                self.update_shader_params();
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.confine_input && self.base().is_visible() {
            if let Ok(mouse_event) = event.try_cast::<InputEventMouse>() {
                if !self.current_rect.has_point(mouse_event.get_global_position()) {
                    if let Some(mut viewport) = self.base().get_viewport() {
                        viewport.set_input_as_handled();
                    }
                } 
            }
        }
    }

    fn on_notification(&mut self, what: notify::ControlNotification) {
        match what {
            notify::ControlNotification::EDITOR_PRE_SAVE => {
                self.base_mut().set_material(None as Option<Gd<ShaderMaterial>>);
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
    // region: Getters/Setters

    // region: Background Color

    #[func]
    fn get_background_color(&self) -> Color {
        self.background_color
    }

    #[func]
    fn set_background_color(&mut self, background_color: Color) {
        self.background_color = background_color;
        self.update_shader_params();
    }

    // endregion

    // region: Border Color

    #[func]
    fn get_border_color(&self) -> Color {
        self.border_color
    }

    #[func]
    fn set_border_color(&mut self, border_color: Color) {
        self.border_color = border_color;
        self.update_shader_params();
    }

    // endregion

    // region: Border Width

    #[func]
    fn get_border_width(&self) -> i32 {
        self.border_width
    }

    #[func]
    fn set_border_width(&mut self, border_width: i32) {
        self.border_width = border_width;
        self.update_shader_params();
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
        self.update_shader_params();
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
        self.update_shader_params();
    }

    // endregion

    // endregion

    fn update_shader_params(&mut self) {
        let padded_rect = self.current_rect.grow(self.padding as f32);
        self.cutout_material.set_shader_parameter("rect_size".into(), padded_rect.size.to_variant());
        self.cutout_material.set_shader_parameter("rect_position".into(), padded_rect.position.to_variant());
        self.cutout_material.set_shader_parameter("corner_radius".into(), self.corner_radius.to_variant());
        self.cutout_material.set_shader_parameter("background_color".into(), self.background_color.to_variant());
        self.cutout_material.set_shader_parameter("border_color".into(), self.border_color.to_variant());
        self.cutout_material.set_shader_parameter("border_width".into(), self.border_width.to_variant());
    }
}
