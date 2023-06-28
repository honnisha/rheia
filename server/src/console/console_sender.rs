use log::info;
use std::fmt::{self, Display};

pub trait ConsoleSender {
    fn send_console_message(&self, message: String);
}

pub trait ConsoleSenderType: ConsoleSender + Display {}

impl Display for dyn ConsoleSender {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ConsoleSender")
    }
}

#[derive(Default, Clone)]
pub struct Console;

impl ConsoleSender for Console {
    fn send_console_message(&self, message: String) {
        info!("{}", message)
    }
}
impl ConsoleSenderType for Console {}

impl Display for Console {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Console")
    }
}
