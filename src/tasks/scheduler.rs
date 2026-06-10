use tokio::time::{sleep, Duration};

pub struct RuntimeScheduler;

impl RuntimeScheduler {
    pub async fn heartbeat() {
        loop {
            println!("[scheduler] heartbeat");

            sleep(Duration::from_secs(5)).await;
        }
    }
}