use std::io::Write;

use crate::{progress_bar::print_progress, simulator::Simulator, statistics::DataFrame, CONFIG};

use super::Renderer;

pub struct NoGraphics {
    pub simulator: Simulator,
}

impl Renderer for NoGraphics {
    fn attach(&mut self, simulator: Simulator) {
        self.simulator = simulator;
    }

    fn run(&mut self, debug: bool, show_progress: bool, export: bool) {
        let mut dataframe = DataFrame::new(CONFIG.core.population_size as usize);
        dataframe.push_data(&self.simulator);

        for i in 0..CONFIG.core.time_limit {
            if show_progress {
                let progress = i as f32 / CONFIG.core.time_limit as f32 * 100.0;
                print_progress(progress);
                print!("\r");
            }

            self.simulator.step();

            dataframe.push_data(&self.simulator);
        }

        if debug {
            println!("{}", dataframe);
        }

        if export {
            println!("{}", std::env::current_dir().unwrap().display());
            // Save data frame as csv file.
            let mut file = std::fs::File::create(format!(
                "export/{}_{}.csv",
                CONFIG.name().split("/").last().unwrap(),
                chrono::offset::Local::now().format("%Y-%m-%d_%H-%M-%S")
            ))
            .expect("Unable to create file");

            file.write_all(dataframe.to_csv().as_bytes())
                .expect("Unable to write to file");
        }
    }
}

impl NoGraphics {
    pub fn new(simulator: Simulator) -> NoGraphics {
        NoGraphics { simulator }
    }
}
