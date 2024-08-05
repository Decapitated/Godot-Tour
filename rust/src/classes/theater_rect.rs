use godot::prelude::*;
use godot::classes::{TextureRect, ITextureRect, Image, image, ImageTexture, Control, InputEvent, InputEventMouse, ReferenceRect, notify};

#[derive(GodotClass)]
#[class(base = TextureRect, tool)]
struct TheaterRect {
    base: Base<TextureRect>,
    #[export]
    focused_node: Option<Gd<Control>>,
    #[export]
    reference_rect: Option<Gd<ReferenceRect>>,
    #[export] #[var(get = get_dim_color, set = set_dim_color)]
    dim_color: Color,
    #[export] #[var(get = get_padding, set = set_padding)]
    padding: i32,
    #[export] confine_input: bool,
    cutout_image: Option<Gd<Image>>,
    cutout_texture: Option<Gd<ImageTexture>>,
    current_rect: Rect2,
}

const CUTOUT_COLOR: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.0);

#[godot_api]
impl ITextureRect for TheaterRect {
    fn init(base: Base<TextureRect>) -> Self {
        Self {
            base,
            focused_node: None,
            reference_rect: None,
            dim_color: Color::from_rgba(0.0, 0.0, 0.0, 0.666666),
            padding: 16,
            confine_input: true,
            cutout_image: None,
            cutout_texture: None,
            current_rect: Rect2::default(),
        }
    }

    fn ready(&mut self) {
        self.create_image();
        let on_resize = self.base_mut().callable("on_resize");
        self.base_mut().connect("resized".into(), on_resize);
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
                self.base_mut().set_texture(None as Option<Gd<ImageTexture>>);
            },
            notify::ControlNotification::EDITOR_POST_SAVE => {
                let clone_texture = self.cutout_texture.clone();
                self.base_mut().set_texture(clone_texture);
            },
            _ => {}
        }
    }
}

#[godot_api]
impl TheaterRect {
    // region: Getters/Setters

    // region: Padding

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

    #[func]
    fn on_resize(&mut self) {
        self.create_image();
    }

    fn update(&mut self) {
        self.draw_cutout();
        self.update_reference_rect();
    }

    fn create_image(&mut self) {
        let size = self.base().get_size();
        self.cutout_image = Image::create(size.x as i32, size.y as i32, false, image::Format::RGBA8);
        self.cutout_texture = ImageTexture::create_from_image(self.cutout_image.clone());
        let clone_texture = self.cutout_texture.clone();
        self.base_mut().set_texture(clone_texture);
    }

    fn draw_cutout(&mut self) {
        if let Some(image) = self.cutout_image.as_mut() {
            if let Some(texture) = self.cutout_texture.as_mut() {
                image.fill(self.dim_color);
                let padded_rect = self.current_rect.grow(self.padding as f32);
                image.fill_rect(padded_rect.cast_int(), CUTOUT_COLOR);
                texture.update(image.clone());
            }
        }
    }

    fn update_reference_rect(&mut self) {
        if let Some(mut reference_rect) = self.reference_rect.clone() {
            let padded_rect = self.current_rect.grow(self.padding as f32);
            reference_rect.set_global_position(padded_rect.position);
            reference_rect.set_size(padded_rect.size);
        }
    }
}
