use std::fmt::Display;

use plotters::{
    backend::RGBPixel,
    coord::types::RangedCoordu32,
    prelude::{
        BitMapBackend, Cartesian2d, ChartBuilder, ChartContext, IntoDrawingArea, LineSeries,
        PathElement,
    },
    style::{full_palette::GREY, Color, IntoFont, BLACK, GREEN, MAGENTA, RED, WHITE},
};

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
            simulator.hospital().lock().unwrap().count(),
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

        // TODO write macro to shorten this code and get rid of the functions for all the members
        // This will also make it easier to add new members to the dataframe.
        self.draw_susceptible_line(&mut chart)?;
        self.draw_infected_line(&mut chart)?;
        self.draw_hospitalized_line(&mut chart)?;
        self.draw_recovered_line(&mut chart)?;
        self.draw_dead_line(&mut chart)?;

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .label_font(("sans-serif", 20).into_font())
            .draw()?;

        root.present()?;

        Ok(())
    }

    fn draw_susceptible_line(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordu32, RangedCoordu32>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|datapoint| (datapoint.timestamp, datapoint.susceptible)),
                &GREY,
            ))?
            .label("Susceptible")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREY));

        Ok(())
    }

    fn draw_infected_line(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordu32, RangedCoordu32>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|datapoint| (datapoint.timestamp, datapoint.infected)),
                &RED,
            ))?
            .label("Infected")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        Ok(())
    }

    fn draw_hospitalized_line(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordu32, RangedCoordu32>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|datapoint| (datapoint.timestamp, datapoint.hospitalized)),
                &MAGENTA,
            ))?
            .label("Hospitalized")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

        Ok(())
    }

    fn draw_recovered_line(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordu32, RangedCoordu32>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|datapoint| (datapoint.timestamp, datapoint.recovered)),
                &GREEN,
            ))?
            .label("Recovered")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        Ok(())
    }

    fn draw_dead_line(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordu32, RangedCoordu32>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|datapoint| (datapoint.timestamp, datapoint.dead)),
                &BLACK,
            ))?
            .label("Dead")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

        Ok(())
    }
}
