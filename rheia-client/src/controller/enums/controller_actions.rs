pub enum ControllerActions {
    ActionMain,
    ActionSecond,

    SwitchCameraMode,

    MoveRight,
    MoveLeft,
    MoveForward,
    MoveBackwards,

    RotateLeft,
    RotateRight,

    CancelSelection,
    Escape,

    ToggleDebug,
    ToggleConsole,
    ToggleBlockSelection,
    Jump,
}

impl ToString for ControllerActions {
    fn to_string(&self) -> String {
        let s = match self {
            Self::ActionMain => "action_main",
            Self::ActionSecond => "action_second",

            Self::SwitchCameraMode => "switch_camera_mode",

            Self::MoveRight => "move_right",
            Self::MoveLeft => "move_left",
            Self::MoveForward => "move_forward",
            Self::MoveBackwards => "move_backwards",

            Self::RotateLeft => "rotate_left",
            Self::RotateRight => "rotate_right",

            Self::CancelSelection => "cancel_selection",
            Self::Escape => "escape",

            Self::ToggleDebug => "toggle_debug",
            Self::ToggleConsole => "toggle_console",
            Self::ToggleBlockSelection => "toggle_block_selection",
            Self::Jump => "jump",
        };
        s.to_string()
    }
}
