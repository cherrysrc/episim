use config::Config;
use graphics::Renderer;
use lazy_static::lazy_static;
use quadtree::Positioned;
use rand_distr::Normal;

mod config;
mod entity;
mod graphics;
mod hospital;
mod progress_bar;
mod simulator;
mod statistics;
mod unsafe_array;

lazy_static! {
    pub static ref CONFIG: Config = Config::new(
        "configurations/example_config1.ron",
        |entity| { 1.0 - (entity.age() as f32 / 100.0) },
        |a, b| {
            let dist = a.position().distance(b.position());
            1.0 / dist
        },
        Normal::new(0.0, 1.0).unwrap(),
    )
    .unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simulator = simulator::Simulator::new(num_cpus::get() as u32);

    // If you dont need the graphics, you can use NoGraphics.
    // let mut renderer = graphics::NoGraphics::new(simulator);
    let mut renderer = graphics::SDL::new(simulator);

    renderer.run(true, true, true);

    Ok(())
}
