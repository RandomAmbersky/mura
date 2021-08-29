mod logger;

use std::time::Instant;
use crate::interfaces::ILogger;

pub struct Logger {
    start: Instant
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.log("Logger", "Drop")
    }
}

impl ILogger for Logger {
    fn new () -> Self {
        Self {
            start: Instant::now()
        }
    }
    fn log(&mut self, service: &str, message: &str) {
        let elapsed = self.start.elapsed();
        self.start = Instant::now();
        println!("{:?}| {}: {}", elapsed, service, message);
    }
}
