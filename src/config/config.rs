use rand::prelude::StdRng;

use crate::entity::Entity;

use super::{age_distribution::AgeDistribution, core::ConfigCore};

//https://www.populationpyramid.net/

pub struct Config {
    name: String,
    pub survival_chance: fn(&Entity) -> f32, // Calculates the survival chance of an entity.
    pub infection_chance: fn(&Entity, &Entity) -> f32, // Calculates the chance entity a will infect entity b.

    age_distribution: AgeDistribution,

    // Everything that can be serialzed
    pub core: ConfigCore,
}

impl Config {
    /// Load a config from a file.
    /// File needs to be in ron format.
    /// Functions, rngs and distributions cannot be serialized and therefore have to be given explicitly.
    /// TODO lua support?
    pub fn new(
        name: &'static str,
        survival_chance: fn(&Entity) -> f32,
        infection_chance: fn(&Entity, &Entity) -> f32,
    ) -> Result<Config, Box<dyn std::error::Error>> {
        let core = ConfigCore::load(name)?;
        let age_distribution = AgeDistribution::load(name)?;

        Ok(Config {
            name: format!(
                "{}_{}",
                name,
                chrono::offset::Local::now().format("%Y-%m-%d_%H-%M-%S")
            ),
            survival_chance,
            infection_chance,
            age_distribution,
            core,
        })
    }

    pub fn sample_age(&self, rng: &mut StdRng) -> u8 {
        self.age_distribution.sample(rng)
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
