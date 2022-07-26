use std::fmt::Display;

/// A data point is a all the given statistical information for a single simulation time step.
pub struct DataPoint {
    timestamp: u32,
    susceptible: u32,
    infected: u32,
    hospitalized: u32,
    recovered: u32,
    dead: u32,
}

impl Display for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Timestamp: {}, Susceptible: {}, Infected: {}, Hospitalized: {}, Recovered: {}, Dead: {}]\n",
            self.timestamp, self.susceptible, self.infected, self.hospitalized, self.recovered, self.dead
        )
    }
}

impl DataPoint {
    pub fn new(
        timestamp: u32,
        susceptible: u32,
        infected: u32,
        hospitalized: u32,
        recovered: u32,
        dead: u32,
    ) -> DataPoint {
        DataPoint {
            timestamp,
            susceptible,
            infected,
            hospitalized,
            recovered,
            dead,
        }
    }

    pub fn as_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{}\n",
            self.timestamp,
            self.susceptible,
            self.infected,
            self.hospitalized,
            self.recovered,
            self.dead
        )
    }
}
