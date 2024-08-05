use godot::prelude::*;
use godot::classes::{MarginContainer, IMarginContainer, Control, notify};

#[derive(GodotClass)]
#[class(base = MarginContainer, tool)]
struct PopupRect {
    base: Base<MarginContainer>,
    #[export]
    target: Option<Gd<Control>>,
}

#[godot_api]
impl IMarginContainer for PopupRect {
    fn init(base: Base<MarginContainer>) -> Self {
        Self {
            base,
            target: None,
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
}

#[godot_api]
impl PopupRect {
    fn reset(&mut self) {
        self.base_mut().set_size(Vector2::default());
        self.base_mut().set_position(Vector2::default());
    }

    fn update(&mut self) {
        self.update_position();
    }

    fn update_position(&mut self) {
        if let Some(target) = self.target.clone() {
            let global_rect = target.get_global_rect();
            self.base_mut().set_position(global_rect.position + Vector2::new(global_rect.size.x, 0.0));
        }
    }
}
