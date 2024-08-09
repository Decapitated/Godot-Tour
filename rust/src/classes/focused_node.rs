use godot::prelude::*;
use godot::builtin::NodePath;
use godot::classes::{Resource, IResource};

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct FocusedNode {
    base: Base<Resource>,
    /// Node to focus on.
    #[export]
    #[var(hint = NODE_TYPE, hint_string = "Control")]
    pub target: NodePath,
    /// Overlay node for borders and other styles/effects.
    #[export]
    #[var(hint = NODE_TYPE, hint_string = "Panel")]
    pub overlay: NodePath,
}

#[godot_api]
impl IResource for FocusedNode {
    fn init(base: Base<Resource>) -> Self {
        // Set default values.
        Self {
            base,
            target: NodePath::default(),
            overlay: NodePath::default(),
        }
    }
}

#[godot_api]
impl FocusedNode {}
