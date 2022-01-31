use lightning::util::logger::*;
use std::sync::Arc;

pub struct LnCtlLogger {}

pub fn init_logger() -> Arc<LnCtlLogger> {
    Arc::new(LnCtlLogger {})
}

impl Logger for LnCtlLogger {
    fn log(&self, record: &Record<'_>) {
        if record.level == Level::Error {
            eprintln!("{}", record.args);
        } else {
            println!("{}", record.args);
        }
    }
}
