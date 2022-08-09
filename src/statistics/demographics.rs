use std::fmt::Display;

use plotters::{
    prelude::{
        BitMapBackend, ChartBuilder, IntoDrawingArea, IntoSegmentedCoord, Rectangle, SegmentValue,
    },
    style::{Color, IntoFont, RED, WHITE},
};

use crate::CONFIG;

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

    fn max_bucket(&self) -> u32 {
        let mut max = 0;
        for bucket in &self.buckets {
            if bucket.count > max {
                max = bucket.count;
            }
        }
        max
    }

    pub fn export(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filename = format!("export/{}/demographics.png", CONFIG.name());

        let root = BitMapBackend::new(&filename, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Demographics", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d((0..100 as u32).into_segmented(), 0..self.max_bucket())?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .x_label_style(("sans-serif", 25).into_font())
            .x_desc("Age")
            .y_label_style(("sans-serif", 25).into_font())
            .y_desc("Count")
            .draw()?;

        chart.draw_series(self.buckets.iter().map(|bucket| {
            Rectangle::new(
                [
                    (SegmentValue::Exact(bucket.age as u32), 0),
                    (SegmentValue::Exact(bucket.age as u32 + 1), bucket.count),
                ],
                RED.filled(),
            )
        }))?;

        root.present()?;

        Ok(())
    }
}
