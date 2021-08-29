use crate::logger::{Logger};
use crate::interfaces::ILogger;

pub struct Engine {
    logger: Logger
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.log("Drop");
    }
}

impl Engine {
    pub fn new () -> Self {
        let logger = Logger::new();
        let mut e = Self {
            logger
        };
        e.log("New");
        e
    }
    pub fn log (&mut self, mess: &str) {
        self.logger.log("Engine", mess);
    }
}
