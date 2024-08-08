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
            default_position: PopupPosition::RightCenter,
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
        if let Some(target) = self.target.clone() {
            let global_rect = target.get_global_rect();
            self.base_mut().set_position(global_rect.position + Vector2::new(global_rect.size.x, 0.0));
        }
    }

    fn update_size(&mut self) {
        let children = self.base().get_children();
        for child in children.iter_shared() {
            if let Ok(control) = child.try_cast::<Control>() {
                self.base_mut().set_size(control.get_size());
            }
        }
    }
}
