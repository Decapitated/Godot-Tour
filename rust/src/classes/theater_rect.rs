use godot::prelude::*;
use godot::classes::{TextureRect, ITextureRect, Image, image, ImageTexture, Control};

#[derive(GodotClass)]
#[class(base = TextureRect)]
struct TheaterRect {
    base: Base<TextureRect>,
    #[export]
    focused_node: NodePath,
    #[export]
    dim_color: Color,
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
            focused_node: NodePath::default(),
            dim_color: Color::from_rgba(0.0, 0.0, 0.0, 0.333333),
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
        if !self.focused_node.is_empty() {
            let control_result: Option<Gd<Control>> = self.base().try_get_node_as(self.focused_node.clone());
            if let Some(focused_control) = control_result {
                let global_rect = focused_control.get_global_rect();
                if self.current_rect != global_rect {
                    self.current_rect = global_rect;
                    self.draw_cutout()
                }
            }
        }
    }
}

#[godot_api]
impl TheaterRect {
    #[func]
    fn on_resize(&mut self) {
        self.create_image();
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
                image.fill_rect(self.current_rect.cast_int(), CUTOUT_COLOR);
                texture.update(image.clone());
            }
        }
    }
}
