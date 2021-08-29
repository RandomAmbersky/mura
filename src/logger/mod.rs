use std::time::Instant;

pub struct Logger {
    start: Instant
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.log("Logger", "Drop")
    }
}

impl Logger {
    pub fn new () -> Self {
        Self {
            start: Instant::now()
        }
    }
    pub fn log(&self, service: &str, message: &str) {
        let elapsed = self.start.elapsed();
        // self.start = Instant::now();
        println!("{:?}| {}: {}", elapsed, service, message);
    }
}
