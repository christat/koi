use chrono::Local;
//----------------------------------------------------------------------------------------------------------------------

use env_logger::filter::{Builder, Filter};
use log::{Log, Metadata, Record, SetLoggerError};
//----------------------------------------------------------------------------------------------------------------------

const FILTER_ENV: &str = "LOG_LEVEL";
//----------------------------------------------------------------------------------------------------------------------

pub struct Logger {
    inner: Filter,
}
//----------------------------------------------------------------------------------------------------------------------

impl Logger {
    fn new() -> Self {
        let mut builder = Builder::from_env(FILTER_ENV);
        Self {
            inner: builder.build(),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn init() -> Result<(), SetLoggerError> {
        let logger = Self::new();

        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }
    //------------------------------------------------------------------------------------------------------------------

    fn log(&self, record: &Record) {
        if self.inner.matches(record) {
            println!(
                "[{}][{}] - {}",
                Local::now().format("%d.%m.%Y - %H:%M:%S.%6f"),
                record.level(),
                record.args()
            );
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    fn flush(&self) {}
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
