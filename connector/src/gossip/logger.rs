use lightning::util::logger::*;

pub struct LnLogger {}

impl LnLogger {
    pub fn new() -> Self {
        Self {}
    }
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
