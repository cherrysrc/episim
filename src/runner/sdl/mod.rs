use quadtree::Positioned;
use rusty_gl::{color, shapes::CustomShape2D, vertices::Vertex};

use crate::{
    entity::{Entity, InfectionStatus},
    simulator::Simulator,
    statistics::{DataFrame, Demographics},
    util::print_progress,
    CONFIG,
};

use super::runner::Runner;

/// Function to map a health status to a color.
fn health_to_color(health: &InfectionStatus) -> [f32; 3] {
    match health {
        InfectionStatus::Susceptible => [1.0, 1.0, 1.0],
        InfectionStatus::Infected(_) => [1.0, 0.0, 0.0],
        InfectionStatus::Recovered(_) => [0.0, 1.0, 0.0],
        InfectionStatus::Dead => [0.25, 0.25, 0.25],
    }
}

/// Implement a sdl rendering backend.
pub struct SDL {
    pub simulator: Simulator,
}

fn entity_to_vertex(entity: &Entity) -> Vertex {
    let pos = entity.position();

    Vertex::new(
        [pos.x, pos.y, 0.0].into(),
        health_to_color(entity.health()).into(),
        [0.0, 0.0].into(),
    )
}

impl Runner for SDL {
    fn new(simulator: Simulator) -> SDL {
        SDL { simulator }
    }

    fn run(&mut self, debug: bool, show_progress: bool, export: bool) {
        let sdl = sdl2::init().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        let video_subsystem = sdl.video().unwrap();
        let gl_attrib = video_subsystem.gl_attr();

        gl_attrib.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attrib.set_context_version(4, 5);

        let window = rusty_gl::Window::new()
            .dimensions(CONFIG.core.dimensions.0, CONFIG.core.dimensions.1)
            .title("Episim")
            .build(&video_subsystem)
            .unwrap();

        if debug {
            rusty_gl::debug::enable();
        }

        let mut dataframe = DataFrame::new(CONFIG.core.population_size as usize);
        dataframe.push_data(&self.simulator);

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            if show_progress {
                let progress =
                    self.simulator.current_time() as f32 / CONFIG.core.time_limit as f32 * 100.0;
                print_progress(progress);
                print!("\r");
            }

            window.clear(color::BLACK);

            self.simulator.step();

            let mut vertices = Vec::new();
            for entity in self.simulator.population().get() {
                let vertex = entity_to_vertex(entity);
                vertices.push(vertex);
            }

            let shape = CustomShape2D::new(vertices, gl::POINTS);
            window.draw(&shape);

            dataframe.push_data(&self.simulator);
            window.gl_swap();

            if self.simulator.done() {
                break 'main;
            }
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
                    CONFIG.core.export().expect("Failed to export core config.");
                }
                Err(e) => {
                    println!("Failed to create export directory: {}", e);
                    return;
                }
            }
        }
    }
}
