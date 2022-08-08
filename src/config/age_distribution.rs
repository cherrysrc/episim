use std::fs;

use rand::{rngs::StdRng, Rng};

use super::age_pdf::AgePDF;

// https://en.wikipedia.org/wiki/Pseudo-random_number_sampling
// Given a probability distribution f, the intervals are
// [0, f(1)), [f(1), f(1)+f(2)), ..., [f(n-2) + f(n-1), f(n-1) + f(n))
pub struct AgeDistribution {
    intervals: Vec<(f32, f32)>,
}

impl AgeDistribution {
    pub fn load(name: &'static str) -> Result<AgeDistribution, Box<dyn std::error::Error>> {
        let ages_csv = fs::read_to_string(format!("./configurations/{}/demographic.csv", name))?;
        let age_pdf = AgePDF::load(ages_csv)?;
        let mut intervals = Vec::new();

        for i in 1..(age_pdf.age_chances.len() - 1) {
            let lower_bound = AgeDistribution::sum_to(&age_pdf, i - 1);
            let upper_bound = AgeDistribution::sum_to(&age_pdf, i);

            intervals.push((lower_bound, upper_bound));
        }
        println!("{:?}", intervals);

        Ok(AgeDistribution { intervals })
    }

    fn sum_to(age_pdf: &AgePDF, i: usize) -> f32 {
        let mut sum = 0.0;
        for j in 0..i {
            sum += age_pdf.age_chances[j];
        }
        sum
    }

    pub fn sample(&self, rng: &mut StdRng) -> u8 {
        let rng_value = rng.gen();
        for i in 0..self.intervals.len() {
            let interval = &self.intervals[i];
            if interval.0 <= rng_value && rng_value < interval.1 {
                return i as u8;
            }
        }

        0
    }
}
