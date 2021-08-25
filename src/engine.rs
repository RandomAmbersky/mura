pub struct Engine {}

impl Drop for Engine {
    fn drop(&mut self) {
        self.log("Drop")
    }
}

impl Engine {
    pub fn new () -> Self {
        let e = Self {};
        e.log("New");
        e
    }
    pub fn log (&self, mess: &str) {
        println!("Engine log: {}", mess);
    }
}
