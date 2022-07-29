use crate::{
    simulator::Simulator,
    statistics::{DataFrame, Demographics},
    util::print_progress,
    CONFIG,
};

use super::Runner;

pub struct NoGraphics {
    pub simulator: Simulator,
}

impl Runner for NoGraphics {
    fn new(simulator: Simulator) -> NoGraphics {
        NoGraphics { simulator }
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

        let demographics = Demographics::from_simulator(&self.simulator);

        if debug {
            println!("{}", dataframe);
            println!("{}", demographics);
        }

        if export {
            match std::fs::create_dir_all(format!("export/{}", CONFIG.name())) {
                Ok(_) => {
                    dataframe.export().expect("Failed to export dataframe.");
                    demographics
                        .export()
                        .expect("Failed to export demographics.");
                }
                Err(e) => {
                    println!("Failed to create export directory: {}", e);
                    return;
                }
            }
        }
    }
}
