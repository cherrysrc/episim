use std::fmt::Display;

pub use crate::simulator::Simulator;

use super::DataPoint;

pub struct DataFrame {
    datapoints: Vec<DataPoint>,
}

impl Display for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for datapoint in &self.datapoints {
            write!(f, "{}", datapoint)?;
        }
        Ok(())
    }
}

impl DataFrame {
    pub fn new(size: usize) -> DataFrame {
        DataFrame {
            datapoints: Vec::with_capacity(size),
        }
    }

    pub fn push_data(&mut self, simulator: &Simulator) {
        let mut susceptible = 0 as u32;
        let mut infected = 0 as u32;
        let mut recovered = 0 as u32;
        let mut dead = 0 as u32;

        for entity in simulator.population().get() {
            match entity.health() {
                crate::entity::InfectionStatus::Susceptible => susceptible += 1,
                crate::entity::InfectionStatus::Infected(_) => infected += 1,
                crate::entity::InfectionStatus::Recovered(_) => recovered += 1,
                crate::entity::InfectionStatus::Dead => dead += 1,
            }
        }

        self.datapoints.push(DataPoint::new(
            simulator.current_time(),
            susceptible,
            infected,
            recovered,
            dead,
        ));
    }
}
