use std::{fs::File, io::Read};

use ron::de::from_str;
use serde::{Deserialize, Serialize};

/// This struct contains all the simulation parameters that can be serialized/deserialized.
#[derive(Default, Serialize, Deserialize)]
pub struct ConfigCore {
    pub time_limit: u32,        // Maximum number of days to simulate.
    pub dimensions: (u32, u32), // Simulation space dimensions.
    pub max_velocity: f32,      // Maximum velocity of an entity.

    pub population_size: u32,   // Number of entities in the simulation.
    pub infected_period: u32,   // Number of days an entity is infected.
    pub recovered_period: u32,  // Number of days an entity is recovered.
    pub infection_radius: u32,  // Radius of the infection area.
    pub hospital_period: u32, // Number of days an entity is hospitalized. If the entity is infected already, the entity will only be hospitalized for the remaining infection period.
    pub hospital_capacity: u32, // Maximum number of entities that can be hospitalized at a given time.

    pub initial_infected: f32, // Chance of an entity being infected at the start of the simulation.
    pub initial_mobile: f32,   // Chance of an entity being mobile at the start of the simulation.

    pub tests_per_time: u32,     // Number of tests per day.
    pub test_true_positive: f32, // Chance of a test being true positive. Implies test_false_positive = 1.0 - test_true_positive.
    pub test_true_negative: f32, // Chance of a test being true negative. Implies test_false_negative = 1.0 - test_true_negative.

    pub distancing: bool, // Whether or not distancing is enabled.
}

impl ConfigCore {
    pub fn load(name: &str) -> Result<ConfigCore, Box<dyn std::error::Error>> {
        let mut file = File::open(format!("./configurations/{}/core.cfg", name))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = match from_str::<ConfigCore>(&contents) {
            Ok(cfg) => cfg,
            Err(e) => return Err(e)?,
        };

        Ok(config)
    }
}
