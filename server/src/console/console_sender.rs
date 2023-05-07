use crate::console_send;

pub trait ConsoleSender {
    fn get_name(&self) -> &String;
    fn send_console_message(&self, message: String);
}

pub struct Console {
    name: String,
}

impl Console {
    pub fn init() -> Self {
        Console {
            name: "Console".to_string(),
        }
    }
}

impl ConsoleSender for Console {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn send_console_message(&self, message: String) {
        console_send(message)
    }
}
