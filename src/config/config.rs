use rand::prelude::StdRng;
use rand_distr::{Distribution, Normal};

use crate::entity::Entity;

use super::core::ConfigCore;

pub struct Config {
    pub survival_chance: fn(&Entity) -> f32, // Calculates the survival chance of an entity.
    pub infection_chance: fn(&Entity, &Entity) -> f32, // Calculates the chance entity a will infect entity b.

    age_distribution: Normal<f32>,

    // Everything that can be serialzed
    pub core: ConfigCore,
}

impl Config {
    /// Load a config from a file.
    /// File needs to be in ron format.
    /// Functions, rngs and distributions cannot be serialized and therefore have to be given explicitly.
    /// TODO lua support?
    pub fn new(
        path: &str,
        survival_chance: fn(&Entity) -> f32,
        infection_chance: fn(&Entity, &Entity) -> f32,
        age_distribution: Normal<f32>,
    ) -> Result<Config, Box<dyn std::error::Error>> {
        let core = ConfigCore::load(path)?;

        Ok(Config {
            survival_chance,
            infection_chance,
            age_distribution,
            core,
        })
    }

    pub fn sample_age(&self, rng: &mut StdRng) -> u8 {
        // Sample from the age distribution.
        let age = self.age_distribution.sample(rng);
        return age as u8;
    }
}
