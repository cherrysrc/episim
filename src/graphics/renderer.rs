use crate::simulator::Simulator;

/// Trait representing a something capable of rendering a simulation.
pub trait Renderer {
    fn attach(&mut self, simulator: Simulator);
    fn run(&mut self, debug: bool, show_progress: bool);
}
