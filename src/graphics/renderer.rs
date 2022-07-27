use crate::simulator::Simulator;

/// Trait representing a something capable of rendering a simulation.
pub trait Renderer {
    fn new(simulator: Simulator) -> Self;
    fn run(&mut self, debug: bool, show_progress: bool, export: bool);
}
