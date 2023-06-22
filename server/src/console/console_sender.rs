use log::info;
use std::fmt;

pub trait ConsoleSender {
    fn send_console_message(&self, message: String);
}

#[derive(Default)]
pub struct Console;

impl ConsoleSender for Console {
    fn send_console_message(&self, message: String) {
        info!("{}", message)
    }
}

impl fmt::Debug for Console {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Console")
    }
}
