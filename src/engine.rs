use crate::logger::Logger;

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
        let e = Self {
            logger
        };
        e.log("New");
        e
    }
    pub fn log (&self, mess: &str) {
        self.logger.log("Engine", mess);
    }
}
