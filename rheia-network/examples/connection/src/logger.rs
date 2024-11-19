use log::{Metadata, Record};

use crate::console::Console;

pub(crate) static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

pub(crate) struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!(
                "&8{} {}{}&r: {}",
                record.metadata().target(),
                record.level(),
                record.level(),
                record.args().to_string()
            );
            Console::send(msg);
        }
    }

    fn flush(&self) {}
}
