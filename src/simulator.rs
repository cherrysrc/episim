use std::{sync::Arc, time::Instant};

use quadtree::{Positioned, Quadtree, Rectangle};

use crate::{
    entity::Entity, progress_bar::print_progress, statistics::DataFrame, unsafe_array::UnsafeArray,
    CONFIG,
};

use crossbeam::thread;

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

        thread::scope(|scope| {
            let mut thread_handles = Vec::new();

            for thread_index in 0..self.threads {
                let arr = self.population.clone();
                let q_ref = &qtree;

                let start_index = thread_index * self.entities_per_thread;
                let end_index = (thread_index + 1) * self.entities_per_thread;

                let infection_radius = CONFIG.core.infection_radius as f32;
                let max_vel = CONFIG.core.max_velocity;
                let delta_time = self.delta_time.clone();

                let handle = scope.spawn(move |_| {
                    for entity_index in start_index..end_index {
                        let entity = arr.get_at_mut(entity_index as usize);
                        let pos = *entity.position();

                        let range = q_ref.query(&Rectangle::new(
                            pos.x,
                            pos.y,
                            infection_radius,
                            infection_radius,
                        ));

                        for other in range {
                            let mut diff = pos - *other.position();
                            diff.clamp_mag(max_vel);
                            entity.apply_force(diff * delta_time);
                        }
                    }
                });

                thread_handles.push(handle);
            }

            for handle in thread_handles {
                handle.join().unwrap();
            }

            for entity in self.population.get_mut() {
                entity.update_movement();
            }
        })
        .unwrap();

        self.time += 1;
    }

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
