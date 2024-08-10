use std::{fmt::{self, Display}, any::Any};

pub trait ConsoleSender {
    fn send_console_message(&self, message: String);
}

pub trait ConsoleSenderType: ConsoleSender + Display {
    fn as_any(&self) -> &dyn Any;
}

impl Display for dyn ConsoleSender {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ConsoleSender")
    }
}

#[derive(Default, Clone)]
pub struct Console;

impl ConsoleSender for Console {
    fn send_console_message(&self, message: String) {
        log::info!(target: "console" ,"{}", message)
    }
}
impl ConsoleSenderType for Console {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Console {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Console")
    }
}
