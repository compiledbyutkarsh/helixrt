pub struct SimulationWorld {
    ticks: u64,
}

impl SimulationWorld {
    pub fn new() -> Self {
        Self {
            ticks: 0,
        }
    }

    pub async fn initialize(&self) {
        println!("simulation world initialized");
    }

    pub async fn update(&mut self) {
        self.ticks += 1;

        if self.ticks % 50 == 0 {
            println!("world tick {}", self.ticks);
        }
    }
}