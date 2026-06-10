pub struct RuntimeLogger;

impl RuntimeLogger {
    pub fn info(message: &str) {
        println!("[info] {}", message);
    }

    pub fn warn(message: &str) {
        println!("[warn] {}", message);
    }
}