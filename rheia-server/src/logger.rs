use log::{Record, Level, Metadata};

use crate::console::console_handler::ConsoleHandler;

pub(crate) static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

pub(crate) struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            ConsoleHandler::send_message(record.args().to_string());
        }
    }

    fn flush(&self) {}
}
