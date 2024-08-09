use godot::prelude::*;
use godot::builtin::NodePath;
use godot::classes::Resource;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct FocusedNode {
    base: Base<Resource>,
    // Node to focus on.
    #[export]
    #[var(
        hint = NODE_TYPE,
        hint_string = "Control"
    )]
    pub target: NodePath,
    // Overlay node for borders and other styles/effects.
    #[export]
    #[var(
        hint = NODE_TYPE,
        hint_string = "Panel"
    )]
    pub overlay: NodePath,
}

// #[godot_api]
// impl IResource for FocusedNode {
//     fn init(base: Base<Resource>) -> Self {
//         // Set default values.
//         Self {
//             base,
//             target: None,
//             overlay: None,
//         }
//     }
// }

#[godot_api]
impl FocusedNode {}
