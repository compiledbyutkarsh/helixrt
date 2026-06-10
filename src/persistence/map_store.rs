use std::fs::File;
use std::io::Write;

pub struct MapStore;

impl MapStore {
    pub fn persist(
        occupancy: f64,
    ) {
        let mut file =
            File::create("runtime_map.snapshot")
                .unwrap();

        let payload = format!(
            "occupancy={:.4}",
            occupancy
        );

        let _ =
            file.write_all(payload.as_bytes());
    }
}