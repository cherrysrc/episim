use std::{sync::Arc, time::Instant};

use crossbeam::thread;
use quadtree::{Positioned, Quadtree, Rectangle};

use crate::{
    entity::{Entity, InfectionStatus}, progress_bar::print_progress, statistics::DataFrame, unsafe_array::UnsafeArray,
    CONFIG,
};

pub struct Simulator {
    population: Arc<UnsafeArray<Entity>>,
    time: u32,

    threads: u32,
    entities_per_thread: u32,

    delta_time: f32,
    frame_timer: Instant,
}

impl Simulator {
    pub fn new(threads: u32) -> Simulator {
        let population = (0..CONFIG.core.population_size)
            .map(|_| Entity::new())
            .collect();

        let entities_per_thread = CONFIG.core.population_size / threads;

        Simulator {
            population: Arc::new(UnsafeArray::new(population)),
            time: 0,
            threads,
            entities_per_thread,
            delta_time: 1.0,
            frame_timer: Instant::now(),
        }
    }

    /// Helper function for iterating over the population in parallel.
    fn for_each_entity(&self, f: &(impl Fn(&mut Entity) + Send + Sync)) {
        thread::scope(|scope| {
            let mut thread_handles = Vec::new();

            for thread_index in 0..self.threads {
                let population = self.population.clone();

                let handle = scope.spawn(move |_| {
                    let start_index = thread_index * self.entities_per_thread;
                    let end_index = (thread_index + 1) * self.entities_per_thread;

                    for i in start_index..end_index {
                        f(population.get_at_mut(i as usize));
                    }
                });

                thread_handles.push(handle);
            }

            for handle in thread_handles {
                handle.join().unwrap();
            }
        })
        .unwrap();
    }

    /// Performs a single simulation time step.
    pub fn step(&mut self) {
        self.delta_time = self.frame_timer.elapsed().as_secs_f32();
        self.frame_timer = Instant::now();

        let mut qtree = Quadtree::new(
            CONFIG.core.dimensions.0 as f32 * 0.5,
            CONFIG.core.dimensions.1 as f32 * 0.5,
            CONFIG.core.dimensions.0 as f32,
            CONFIG.core.dimensions.1 as f32,
        );

        let p = self.population.clone();
        for entity in p.get() {
            // If this fails, it means that our entity and area generation is wrong
            qtree.insert(entity).unwrap();
        }

        self.for_each_entity(&|entity: &mut Entity| {
            let pos = *entity.position();
            let range = qtree.query(&Rectangle::new(
                pos.x,
                pos.y,
                CONFIG.core.infection_radius as f32,
                CONFIG.core.infection_radius as f32,
            ));

            // Apply repulsion force, simulates distancing from other entities
            for other in range {
                let diff = pos - *other.position();
                entity.apply_force(diff * 0.05 * self.delta_time);

                // Only check if other entity is infected and entity itself is susceptible
                match (other.status(), entity.status()) {
                    (InfectionStatus::Infected(_), InfectionStatus::Susceptible) => {
                        if entity.rand() > (CONFIG.infection_chance)(other, entity) {
                            entity.infect();
                        }
                    },
                    _ => {}
                }
            }
        });

        self.for_each_entity(&|entity: &mut Entity| {
            entity.update_status();
            entity.update_movement();
        });

        self.time += 1;
    }

    /// Runs the entire simulation.
    /// Use this if you dont want/need a visalization.
    pub fn run(&mut self, debug: bool, show_progress: bool) {
        let mut dataframe = DataFrame::new(CONFIG.core.population_size as usize);
        dataframe.push_data(self);

        for i in 0..CONFIG.core.time_limit {
            if show_progress {
                let progress = i as f32 / CONFIG.core.time_limit as f32 * 100.0;
                print_progress(progress);
                print!("\r");
            }

            self.step();

            dataframe.push_data(self);
        }

        if debug {
            println!("{}", dataframe);
        }
    }

    pub fn done(&self) -> bool {
        self.time >= CONFIG.core.time_limit
    }

    pub fn current_time(&self) -> u32 {
        self.time
    }

    pub fn population(&self) -> &UnsafeArray<Entity> {
        &self.population
    }
}
