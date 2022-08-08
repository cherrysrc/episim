const BUCKET_COUNT: usize = 20;

pub struct AgePDF {
    pub(super) age_chances: [f32; 101],
}

impl AgePDF {
    pub fn load(ages_csv: String) -> Result<AgePDF, Box<dyn std::error::Error>> {
        let mut age_chances = [0.0; 101];
        let mut lines = ages_csv.lines();
        lines.next(); // Skip header

        let mut population_total = 0.0;

        for bucket in 0..BUCKET_COUNT {
            let line = lines.next().unwrap();
            let parts: Vec<&str> = line.split(",").collect();

            let bucket_count = parts[1].parse::<u32>().unwrap() + parts[2].parse::<u32>().unwrap();
            population_total += bucket_count as f32;
            let bucket_count = bucket_count / 5;

            age_chances[bucket as usize * 5 + 0] = bucket_count as f32;
            age_chances[bucket as usize * 5 + 1] = bucket_count as f32;
            age_chances[bucket as usize * 5 + 2] = bucket_count as f32;
            age_chances[bucket as usize * 5 + 3] = bucket_count as f32;
            age_chances[bucket as usize * 5 + 4] = bucket_count as f32;
        }

        // Special case for 100+ group
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split(",").collect();
        let bucket_count = parts[1].parse::<u32>().unwrap() + parts[2].parse::<u32>().unwrap();
        population_total += bucket_count as f32;
        age_chances[100] = bucket_count as f32;

        for bucket in 0..101 {
            age_chances[bucket as usize] /= population_total;
        }

        Ok(AgePDF { age_chances })
    }
}
