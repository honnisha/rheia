use log::{Metadata, Record};

use crate::console::{
    colors::{get_log_level_color, Color},
    console_handler::ConsoleHandler,
};

pub(crate) static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

pub(crate) struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if metadata.target() == "rustyline" {
            return false;
        }
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            ConsoleHandler::send_message(format!(
                "{}{} {}{}{}: {}",
                Color::DarkGray.to_terminal(),
                record.metadata().target(),
                get_log_level_color(&record.level()).to_terminal(),
                record.level(),
                Color::Reset.to_terminal(),
                record.args().to_string()
            ));
        }
    }

    fn flush(&self) {}
}
