use common::utils::colors::get_log_level_color;
use log::{Metadata, Record};

use crate::console::{
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
                "&8{} {}{}&r: {}",
                record.metadata().target(),
                get_log_level_color(&record.level()),
                record.level(),
                record.args().to_string()
            ));
        }
    }

    fn flush(&self) {}
}
