use std::fmt::Display;

use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries, PathElement},
    style::{full_palette::GREY, Color, IntoFont, BLACK, GREEN, MAGENTA, RED, WHITE},
};

use crate::entity::InfectionStatus;
pub use crate::simulator::Simulator;
use crate::CONFIG;

use super::DataPoint;

/// DataFrame tracks/collects all the data points for a simulation.
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

/// Shortcut for adding a DataFrames datapoints to the chart more easily.
macro_rules! add_chart_line {
    ($chart:expr, $dataframe:expr, $attribute:tt , $color:expr) => {
        $chart
            .draw_series(LineSeries::new(
                $dataframe
                    .datapoints()
                    .iter()
                    .map(|dp| (dp.timestamp, dp.$attribute)),
                $color,
            ))?
            .label(stringify!($attribute))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], $color));
    };
}

impl DataFrame {
    pub fn new(size: usize) -> DataFrame {
        DataFrame {
            datapoints: Vec::with_capacity(size),
        }
    }

    pub fn datapoints(&self) -> &Vec<DataPoint> {
        &self.datapoints
    }

    pub fn push_data(&mut self, simulator: &Simulator) {
        let mut susceptible = 0 as u32;
        let mut infected = 0 as u32;
        let mut recovered = 0 as u32;
        let mut dead = 0 as u32;

        for entity in simulator.population().get() {
            match entity.health() {
                InfectionStatus::Susceptible => susceptible += 1,
                InfectionStatus::Infected(_) => infected += 1,
                InfectionStatus::Recovered(_) => recovered += 1,
                InfectionStatus::Dead => dead += 1,
            }
        }

        self.datapoints.push(DataPoint::new(
            simulator.current_time(),
            susceptible,
            infected,
            simulator.hospital().lock().unwrap().count() as u32,
            recovered,
            dead,
        ));
    }

    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str(
            format!(
                "{},{},{},{},{},{}\n",
                "time", "susceptible", "infected", "hospital", "recovered", "dead"
            )
            .as_str(),
        );
        for datapoint in &self.datapoints {
            csv.push_str(&format!("{}", datapoint.as_csv()));
        }
        csv
    }

    pub fn save_as_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filename = format!(
            "export/{}_{}.png",
            CONFIG.name().split("/").last().unwrap(),
            chrono::offset::Local::now().format("%Y-%m-%d_%H-%M-%S")
        );

        let root = BitMapBackend::new(&filename, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(&filename, ("sans-serif", 20).into_font())
            .margin(5)
            .x_label_area_size(100)
            .y_label_area_size(100)
            .build_cartesian_2d(0..CONFIG.core.time_limit, 0..CONFIG.core.population_size)?;

        chart
            .configure_mesh()
            .x_label_style(("sans-serif", 20).into_font())
            .y_label_style(("sans-serif", 20).into_font())
            .draw()?;

        add_chart_line!(chart, self, susceptible, GREY);
        add_chart_line!(chart, self, infected, RED);
        add_chart_line!(chart, self, hospitalized, MAGENTA);
        add_chart_line!(chart, self, recovered, GREEN);
        add_chart_line!(chart, self, dead, BLACK);

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .label_font(("sans-serif", 20).into_font())
            .draw()?;

        root.present()?;

        Ok(())
    }
}
