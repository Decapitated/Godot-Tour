use godot::prelude::*;
use godot::classes::{ColorRect, IColorRect, Control, InputEvent, InputEventMouse, ReferenceRect, ShaderMaterial, Shader, notify};

#[derive(GodotClass)]
#[class(base = ColorRect, tool)]
struct TheaterRect {
    base: Base<ColorRect>,
    // Node to focus on.
    #[export]
    focused_node: Option<Gd<Control>>,
    // ReferenceRect to outline focused node.
    #[export]
    reference_rect: Option<Gd<ReferenceRect>>,
    // Background color of unfocused area.
    #[export] #[var(get = get_dim_color, set = set_dim_color)]
    dim_color: Color,
    // Padding between unfocused and focused area.
    #[export] #[var(get = get_padding, set = set_padding)]
    padding: i32,
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
            reference_rect: None,
            dim_color: Color::from_rgba(0.0, 0.0, 0.0, 0.9),
            padding: 16,
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
                self.update();
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

    // region: Dim Color

    #[func]
    fn get_dim_color(&self) -> Color {
        self.dim_color
    }

    #[func]
    fn set_dim_color(&mut self, dim_color: Color) {
        self.dim_color = dim_color;
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

    // endregion

    fn update(&mut self) {
        let padded_rect = self.current_rect.grow(self.padding as f32);
        self.update_shader_params(padded_rect);
        self.update_reference_rect(padded_rect);
    }

    fn update_shader_params(&mut self, rect: Rect2) {
        self.cutout_material.set_shader_parameter("dim_color".into(), self.dim_color.to_variant());
        self.cutout_material.set_shader_parameter("rect_size".into(), rect.size.to_variant());
        self.cutout_material.set_shader_parameter("rect_position".into(), rect.position.to_variant());
    }

    fn update_reference_rect(&mut self, rect: Rect2) {
        if let Some(mut reference_rect) = self.reference_rect.clone() {
            reference_rect.set_global_position(rect.position);
            reference_rect.set_size(rect.size);
        }
    }
}
