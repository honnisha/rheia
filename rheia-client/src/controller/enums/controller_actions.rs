pub enum ControllerActions {
    ActionMain,
    ActionSecond,

    MoveRight,
    MoveLeft,
    MoveForward,
    MoveBackwards,

    ToggleDebug,
    ToggleConsole,
    Jump,
}

impl ToString for ControllerActions {
    fn to_string(&self) -> String {
        let s = match self {
            Self::ActionMain => "action_main",
            Self::ActionSecond => "action_second",

            Self::MoveRight => "move_right",
            Self::MoveLeft => "move_left",
            Self::MoveForward => "move_forward",
            Self::MoveBackwards => "move_backwards",

            Self::ToggleDebug => "toggle_debug",
            Self::ToggleConsole => "toggle_console",
            Self::Jump => "jump",
        };
        s.to_string()
    }
}
