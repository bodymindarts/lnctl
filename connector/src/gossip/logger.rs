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
            eprintln!("Gossip - '{}'", record.args);
        } else {
            println!("Gossip - '{}'", record.args);
        }
    }
}
