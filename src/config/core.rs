use std::{fs::File, io::Read};

use ron::de::from_str;
use serde::{Deserialize, Serialize};

/// This struct contains all the simulation parameters that can be serialized/deserialized.
#[derive(Default, Serialize, Deserialize)]
pub struct ConfigCore {
    pub time_limit: u32,
    pub dimensions: (u32, u32),
    pub max_velocity: f32,

    pub population_size: u32,
    pub infected_period: u32,
    pub recovered_period: u32,
    pub infection_radius: u32,
    pub hospital_capacity: u32,

    pub initial_infected: f32,
    pub initial_mobile: f32,

    pub tests_per_time: u32,
    pub test_true_positive: f32,
    pub test_true_negative: f32,
    pub test_false_positive: f32,
    pub test_false_negative: f32,

    pub distancing: bool,
}

impl ConfigCore {
    pub fn load(path: &str) -> Result<ConfigCore, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = match from_str::<ConfigCore>(&contents) {
            Ok(cfg) => cfg,
            Err(e) => return Err(e)?,
        };

        Ok(config)
    }
}
