use crate::{progress_bar::print_progress, simulator::Simulator, statistics::DataFrame, CONFIG};

use super::Renderer;

pub struct NoGraphics {
    pub simulator: Simulator,
}

impl Renderer for NoGraphics {
    fn attach(&mut self, simulator: Simulator) {
        self.simulator = simulator;
    }

    fn run(&mut self, debug: bool, show_progress: bool) {
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
    }
}

impl NoGraphics {
    pub fn new(simulator: Simulator) -> NoGraphics {
        NoGraphics { simulator }
    }
}
