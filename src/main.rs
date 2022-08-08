use config::Config;
use lazy_static::lazy_static;
use quadtree::Positioned;
use runner::NoGraphics;
use runner::Runner;
use runner::SDL;

mod config;
mod entity;
mod hospital;
mod runner;
mod simulator;
mod statistics;
mod util;

lazy_static! {
    pub static ref CONFIG: Config = Config::new(
        "example_conf",
        |entity| {
            if entity.is_hospitalized() {
                0.99
            } else {
                1.0 - (entity.age() as f32 / 200.0)
            }
        },
        |a, b| {
            let dist = a.position().distance(b.position());
            1.0 / (dist * dist)
        },
    )
    .unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simulator = simulator::Simulator::new(num_cpus::get() as u32);

    // If you dont need the graphics, you can use NoGraphics.
    let mut renderer = NoGraphics::new(simulator);
    // let mut renderer = SDL::new(simulator);

    renderer.run(true, true, true);

    Ok(())
}
