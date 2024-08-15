use std::cmp::Ordering;

use godot::prelude::*;
use godot::classes::{control, notify, Container, Control, IContainer};

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

impl PopupPosition {
    fn iterator() -> std::slice::Iter<'static, PopupPosition> {
        static POPUP_POSITIONS: [PopupPosition; 12] = [
            PopupPosition::TopLeft,
            PopupPosition::TopCenter,
            PopupPosition::TopRight,
            PopupPosition::BottomLeft,
            PopupPosition::BottomCenter,
            PopupPosition::BottomRight,
            PopupPosition::LeftTop,
            PopupPosition::LeftCenter,
            PopupPosition::LeftBottom,
            PopupPosition::RightTop,
            PopupPosition::RightCenter,
            PopupPosition::RightBottom,
        ];
        POPUP_POSITIONS.iter()
    }
}

#[derive(GodotClass)]
#[class(base = Container, tool)]
struct PopupContainer {
    base: Base<Container>,
    #[export]
    #[var(hint = NODE_PATH_VALID_TYPES, hint_string = "Control")]
    pub target: NodePath,
    #[export]
    pub position: PopupPosition,
    /// Enable smart positioning.
    #[export]
    pub smart_position: bool,
}

#[godot_api]
impl IContainer for PopupContainer {
    fn init(base: Base<Container>) -> Self {
        Self {
            base,
            target: NodePath::default(),
            position: PopupPosition::RightTop,
            smart_position: true,
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
                self.base_mut().set_custom_minimum_size(Vector2::default());
            },
            notify::ContainerNotification::EDITOR_POST_SAVE => {},
            _ => {}
        }
    }

    fn get_allowed_size_flags_horizontal(&self) -> PackedInt32Array {
        let mut packed_array =  PackedInt32Array::new();
        packed_array.push(control::SizeFlags::SHRINK_BEGIN.ord() as i32);
        packed_array
    }

    fn get_allowed_size_flags_vertical(&self) -> PackedInt32Array {
        let mut packed_array =  PackedInt32Array::new();
        packed_array.push(control::SizeFlags::SHRINK_BEGIN.ord() as i32);
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
        self.base_mut().set_custom_minimum_size(Vector2::default());
        self.update_position();
        let child_restult = self.get_child();
        if let Some(child) = child_restult {
            self.update_size(child.clone());
            self.update_child_control(child.clone());
        }
    }

    fn update_position(&mut self) {
        let popup_position = if self.smart_position { self.get_position_smart() } else { self.get_popup_position(&self.position) };
        self.base_mut().set_position(popup_position);
    }

    fn update_size(&mut self, child: Gd<Control>) {
        self.base_mut().set_size(child.get_size());
    }

    fn update_child_control(&self, mut child: Gd<Control>) {
        // Child control should always be positioned at (0, 0).
        child.set_position(Vector2::default());
        // Child control should size itself. i.e. Set a custom minimum size.
        child.set_size(Vector2::default());
    }

    fn get_position_smart(&self) -> Vector2 {
        if let Some(viewport) = self.base().get_viewport() {
            let viewport_rect = viewport.get_visible_rect();
            let areas = PopupPosition::iterator().map(|position| {
                let popup_rect = self.get_popup_rect(position);
                let area = match self.check_popup_position(viewport_rect, popup_rect) {
                    Some(area) => area,
                    None => -1.0,
                };
                (position, area)
            });
            let max_area = areas.max_by(|x, y| {
                x.1.partial_cmp(&y.1).unwrap_or(Ordering::Equal)
            });
            if let Some((position, _)) = max_area {
                return self.get_popup_position(position);
            }
        }
        Vector2::default()
    }

    fn get_popup_position(&self, position: &PopupPosition) -> Vector2 {
        if let Some(target) = self.base().try_get_node_as::<Control>(self.target.clone()) {
            let global_rect = target.get_global_rect();
            let global_center = global_rect.position + (global_rect.size / 2.0);
            let size = self.base().get_size();
            return match position {
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

    fn get_popup_rect(&self, position: &PopupPosition) -> Rect2 {
        let popup_position = self.get_popup_position(position);
        return Rect2::new(popup_position, self.base().get_size());
    }
    
    fn check_popup_position(&self, viewport_rect: Rect2, popup_rect: Rect2) -> Option<f32> {
        let intersection_option = popup_rect.intersection(viewport_rect);
        if let Some(intersection) = intersection_option {
            return Some(intersection.area());
        }
        None
    }

    fn get_child(&self) -> Option<Gd<Control>> {
        let children = self.base().get_children();
        for child in children.iter_shared() {
            if let Ok(control) = child.try_cast::<Control>() {
                if control.is_visible() {
                    return Some(control);
                }
            }
        }
        None
    }
}
