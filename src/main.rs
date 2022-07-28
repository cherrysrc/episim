use config::Config;
use lazy_static::lazy_static;
use quadtree::Positioned;
use rand_distr::Normal;
use runner::Runner;

mod config;
mod entity;
mod hospital;
mod runner;
mod simulator;
mod statistics;
mod util;

lazy_static! {
    pub static ref CONFIG: Config = Config::new(
        "configurations/example_config1.ron",
        |entity| {
            if entity.is_hospitalized() {
                0.99
            } else {
                1.0 - (entity.age() as f32 / 100.0)
            }
        },
        |a, b| {
            let dist = a.position().distance(b.position());
            1.0 / dist
        },
        Normal::new(0.5, 0.5).unwrap(),
    )
    .unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simulator = simulator::Simulator::new(num_cpus::get() as u32);

    // If you dont need the graphics, you can use NoGraphics.
    let mut renderer = runner::NoGraphics::new(simulator);
    // let mut renderer = graphics::SDL::new(simulator);

    renderer.run(false, true, true);

    Ok(())
}
