use lightning::util::logger::*;

pub struct LnCtlLogger {}

impl Logger for LnCtlLogger {
    fn log(&self, record: &Record<'_>) {
        if record.level == Level::Error {
            eprintln!("{}", record.args);
        } else {
            println!("{}", record.args);
        }
    }
}
