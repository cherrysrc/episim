use std::fmt::Display;

use super::dataframe::Simulator;

struct Bucket {
    age: u8,
    count: u32,
}

pub struct Demographics {
    buckets: Vec<Bucket>,
}

impl Display for Demographics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bucket in &self.buckets {
            writeln!(f, "{}: {}", bucket.age, bucket.count)?;
        }
        Ok(())
    }
}

impl Demographics {
    pub fn from_simulator(simulator: &Simulator) -> Demographics {
        let mut demographics = Demographics {
            buckets: Vec::new(),
        };

        for entity in simulator.population().get() {
            demographics.add(entity.age());
        }

        // Sort by bucket age
        demographics.buckets.sort_by(|a, b| a.age.cmp(&b.age));

        demographics
    }

    fn add(&mut self, age: u8) {
        // Look for an existing bucket
        for bucket in &mut self.buckets {
            if bucket.age == age {
                bucket.count += 1;
                return;
            }
        }

        // There is no existing bucket, so create a new one.
        self.buckets.push(Bucket { age, count: 1 });
    }
}
