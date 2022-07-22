use crate::simulator::Simulator;

pub trait Renderer {
    fn attach(&mut self, simulator: Simulator);
    fn run(&mut self, debug: bool, show_progress: bool);
}