use godot::prelude::*;
use godot::classes::{Container, IContainer, control, Control, notify};

#[derive(GodotConvert, Var, Export)]
#[godot(via = i64)]
enum PopupPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    LeftTop,
    LeftCenter,
    LeftBottom,
    RightTop,
    RightCenter,
    RightBottom,
}

#[derive(GodotClass)]
#[class(base = Container, tool)]
struct PopupContainer {
    base: Base<Container>,
    #[export]
    target: Option<Gd<Control>>,
    #[export]
    default_position: PopupPosition,
}

#[godot_api]
impl IContainer for PopupContainer {
    fn init(base: Base<Container>) -> Self {
        Self {
            base,
            target: None,
            default_position: PopupPosition::RightTop,
        }
    }

    fn ready(&mut self) {
    }

    fn process(&mut self, _delta: f64) {
        self.update();
    }

    fn on_notification(&mut self, what: notify::ContainerNotification) {
        match what {
            notify::ContainerNotification::EDITOR_PRE_SAVE => {
                self.reset();
            },
            notify::ContainerNotification::EDITOR_POST_SAVE => {
                self.update();
            },
            _ => {}
        }
    }

    fn get_allowed_size_flags_horizontal(&self) -> PackedInt32Array {
        let mut packed_array =  PackedInt32Array::new();
        packed_array.push(control::SizeFlags::SHRINK_CENTER.ord() as i32);
        packed_array
    }

    fn get_allowed_size_flags_vertical(&self) -> PackedInt32Array {
        let mut packed_array =  PackedInt32Array::new();
        packed_array.push(control::SizeFlags::SHRINK_CENTER.ord() as i32);
        packed_array
    }
}

#[godot_api]
impl PopupContainer {
    fn reset(&mut self) {
        self.base_mut().set_size(Vector2::default());
        self.base_mut().set_position(Vector2::default());
    }

    fn update(&mut self) {
        self.update_position();
        self.update_size();
    }

    fn update_position(&mut self) {
        let popup_position = self.get_popup_position();
        self.base_mut().set_position(popup_position);
    }

    fn update_size(&mut self) {
        let children = self.base().get_children();
        for child in children.iter_shared() {
            if let Ok(control) = child.try_cast::<Control>() {
                self.base_mut().set_size(control.get_size());
            }
        }
    }

    fn get_popup_position(&self) -> Vector2 {
        if let Some(target) = self.target.clone() {
            let global_rect = target.get_global_rect();
            let global_center = global_rect.position + (global_rect.size / 2.0);
            let size = self.base().get_size();
            return match self.default_position {
                PopupPosition::TopLeft => global_rect.position - Vector2::new(0.0, size.y),
                PopupPosition::TopCenter => Vector2::new(global_center.x, global_rect.position.y) - Vector2::new(size.x / 2.0, size.y),
                PopupPosition::TopRight => Vector2::new(global_rect.position.x + global_rect.size.x, global_rect.position.y) - Vector2::new(size.x, size.y),
                PopupPosition::BottomLeft => Vector2::new(global_rect.position.x, global_rect.position.y + global_rect.size.y),
                PopupPosition::BottomCenter => Vector2::new(global_center.x, global_rect.position.y + global_rect.size.y) - Vector2::new(size.x / 2.0, 0.0),
                PopupPosition::BottomRight => global_rect.position + global_rect.size - Vector2::new(size.x, 0.0),
                PopupPosition::LeftTop => global_rect.position - Vector2::new(size.x, 0.0),
                PopupPosition::LeftCenter => Vector2::new(global_rect.position.x, global_center.y) - Vector2::new(size.x, size.y / 2.0),
                PopupPosition::LeftBottom => Vector2::new(global_rect.position.x, global_rect.position.y + global_rect.size.y) - Vector2::new(size.x, size.y),
                PopupPosition::RightTop => Vector2::new(global_rect.position.x + global_rect.size.x, global_rect.position.y),
                PopupPosition::RightCenter => Vector2::new(global_rect.position.x + global_rect.size.x, global_center.y) - Vector2::new(0.0, size.y / 2.0),
                PopupPosition::RightBottom => global_rect.position + global_rect.size - Vector2::new(0.0, size.y),
            }
        }
        Vector2::default()
    }
}
