use std::fs;

use rand::{rngs::StdRng, Rng};

use super::age_pdf::AgePDF;

// https://en.wikipedia.org/wiki/Pseudo-random_number_sampling
// Given a cumulative probability distribution f, the intervals are
// [0, f(1)), [f(1), f(2)), ..., [ff(n-1), f(n))
pub struct AgeDistribution {
    intervals: Vec<(f32, f32)>,
}

impl AgeDistribution {
    /// Load a distribution from a file.
    /// https://www.populationpyramid.net/
    /// Loads the csv distribution as a pdf.
    /// Generates the intervals required for sampling.
    pub fn load(name: &'static str) -> Result<AgeDistribution, Box<dyn std::error::Error>> {
        let ages_csv = fs::read_to_string(format!("./configurations/{}/demographic.csv", name))?;
        let age_pdf = AgePDF::load(ages_csv)?;
        let mut intervals = Vec::new();

        for i in 1..(age_pdf.age_chances.len() - 1) {
            // Use the sum_to function to 'convert' the pdf to a cdf
            let lower_bound = AgeDistribution::sum_to(&age_pdf, i - 1);
            let upper_bound = AgeDistribution::sum_to(&age_pdf, i);

            intervals.push((lower_bound, upper_bound));
        }

        Ok(AgeDistribution { intervals })
    }

    /// Helper function to sum up the probabilities of the AgePDF
    fn sum_to(age_pdf: &AgePDF, i: usize) -> f32 {
        let mut sum = 0.0;
        for j in 0..i {
            sum += age_pdf.age_chances[j];
        }
        sum
    }

    /// Sample an age from the distribution.
    /// See the wikipedia article linked on top for details.
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
