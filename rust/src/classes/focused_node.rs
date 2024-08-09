use godot::prelude::*;
use godot::builtin::NodePath;
use godot::classes::Resource;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct FocusedNode {
    base: Base<Resource>,
    /// Node to focus on.
    #[export]
    #[var(hint = NODE_PATH_VALID_TYPES, hint_string = "Control")]
    pub target: NodePath,
    /// Overlay node for borders and other styles/effects.
    #[export]
    #[var(hint = NODE_PATH_VALID_TYPES, hint_string = "Panel")]
    pub overlay: NodePath,
}

#[godot_api]
impl FocusedNode {}
