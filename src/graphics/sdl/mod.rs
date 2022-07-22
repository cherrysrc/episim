use quadtree::Positioned;
use rusty_gl::{
    shapes::{CustomShape, Drawable},
    vertices::Vertex,
    PipelineShader, ShaderSource,
};

use crate::{
    entity::Entity, progress_bar::print_progress, simulator::Simulator, statistics::DataFrame,
    CONFIG,
};

use super::renderer::Renderer;

pub struct SDL {
    pub simulator: Simulator,
}

// TODO implement this remap functionality in the rustyGL crate

fn map(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

fn entity_to_vertex(entity: &Entity) -> Vertex {
    let pos = entity.position();

    let mapped_x = map(pos.x, 0.0, CONFIG.core.dimensions.0 as f32, -1.0, 1.0);
    let mapped_y = map(pos.y, 0.0, CONFIG.core.dimensions.1 as f32, 1.0, -1.0);
    
    Vertex::new(
        [mapped_x, mapped_y, 0.0].into(),
        [1.0, 1.0, 1.0].into(),
        [0.0, 0.0].into(),
    )
}

impl Renderer for SDL {
    fn attach(&mut self, simulator: Simulator) {
        self.simulator = simulator;
    }

    fn run(&mut self, debug: bool, show_progress: bool) {
        let sdl = sdl2::init().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        let video_subsystem = sdl.video().unwrap();
        let gl_attrib = video_subsystem.gl_attr();

        gl_attrib.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attrib.set_context_version(4, 5);

        let dimensions = CONFIG.core.dimensions;
        let window = video_subsystem
            .window("Simulator", dimensions.0, dimensions.1)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let _gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        rusty_gl::debug::enable();
        let vertex_shader = Some(ShaderSource::File("src/graphics/sdl/vertex_shader.glsl"));
        let fragment_shader = Some(ShaderSource::File("src/graphics/sdl/fragment_shader.glsl"));

        let shader_bundle = PipelineShader::create(vertex_shader, fragment_shader).unwrap();
        shader_bundle.enable();

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

            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.simulator.step();

            let mut vertices = Vec::new();
            for entity in self.simulator.population().get() {
                let vertex = entity_to_vertex(entity);
                vertices.push(vertex);
            }

            let shape = CustomShape::new(vertices, gl::POINTS, None, None, None);
            shape.draw();

            dataframe.push_data(&self.simulator);
            window.gl_swap_window();

            if self.simulator.done() {
                break 'main;
            }
        }

        if debug {
            println!("{}", dataframe);
        }
    }
}

impl SDL {
    pub fn new(simulator: Simulator) -> SDL {
        SDL { simulator }
    }
}
