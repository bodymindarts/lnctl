use lightning::util::logger::*;
use std::sync::Arc;

pub struct LnLogger {}

pub fn init_logger() -> Arc<LnLogger> {
    Arc::new(LnLogger {})
}

impl Logger for LnLogger {
    fn log(&self, record: &Record<'_>) {
        if record.level == Level::Error {
            eprintln!("{}", record.args);
        } else {
            println!("{}", record.args);
        }
    }
}
