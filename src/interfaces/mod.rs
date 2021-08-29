pub trait ILogger {
    fn new () -> Self;
    fn log (&mut self, service: &str, message: &str);
}
