use quadtree::Positioned;
use rand::{prelude::StdRng, Rng, SeedableRng};
use vector::Vector2;

use crate::CONFIG;

#[derive(PartialEq)]
pub enum InfectionStatus {
    Susceptible,
    Infected(u32),  // number of days the entity will remain infected. The entity will either recover or die.
    Recovered(u32), // number of days the entity will remain recovered (cannot be infected again). The entity will be susceptible again if this counter reaches 0.
    Dead
}

pub struct Entity {
    position: Vector2<f32>,     // Used for calculating entity movement.
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,

    health: InfectionStatus,
    hospitalized: bool,
    mobile: bool,               // True if the entity can move (Neither dead, nor in Hospital). False if it is immobile.

    age: u8,
    survival_chance: f32,       // Chance the entity will survive an infection (move to the recovered status). Based on the age of the entity.

    rng: StdRng,
}

// Required for the quadtree to work.
impl Positioned for Entity {
    fn position(&self) -> &Vector2<f32> {
        &self.position
    }
}

impl Entity {
    pub fn new() -> Entity {
        let mut rng: StdRng = rand::rngs::StdRng::from_entropy();

        let x_position = rng.gen_range(0.0..CONFIG.core.dimensions.0 as f32);
        let y_position = rng.gen_range(0.0..CONFIG.core.dimensions.1 as f32);

        let chance = rng.gen::<f32>();
        let infected = chance < CONFIG.core.initial_infected;

        let chance = rng.gen::<f32>();
        let mobile = chance < CONFIG.core.initial_mobile;

        let age = CONFIG.sample_age(&mut rng);

        let mut entity = Entity {
            position: Vector2::new(x_position, y_position),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            health: if infected {
                InfectionStatus::Infected(CONFIG.core.infected_period)
            } else {
                InfectionStatus::Susceptible
            },
            hospitalized: false,
            mobile,
            age,
            survival_chance: 1.0,
            rng,
        };

        entity.calculate_survival_chance();

        entity
    }

    fn calculate_survival_chance(&mut self) {
        self.survival_chance = (CONFIG.survival_chance)(self)
    }

    /// Simple model for force based movement.
    pub fn update_movement(&mut self) {
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.acceleration *= 0.0;
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.acceleration += force;
    }

    pub fn hospitalize(&mut self) {
        self.hospitalized = true;
        self.mobile = false;
    }

    pub fn is_hospitalized(&self) -> bool {
        self.hospitalized
    }

    pub fn recover(&mut self) {
        self.health = InfectionStatus::Recovered(CONFIG.core.recovered_period);
    }

    pub fn die(&mut self) {
        self.health = InfectionStatus::Dead;
    }

    pub fn infect(&mut self) {
        self.health = InfectionStatus::Infected(CONFIG.core.infected_period);
    }

    pub fn status(&self) -> &InfectionStatus {
        &self.health
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn update_status(&mut self) {
        match self.health {
            InfectionStatus::Infected(time_remaining) => {
                if time_remaining <= 0 {
                    let chance = self.rng.gen::<f32>();

                    if chance <= self.survival_chance {
                        self.recover();
                    } else {
                        self.die();
                    }
                } else {
                    self.health = InfectionStatus::Infected(time_remaining - 1);
                }
            },
            InfectionStatus::Recovered(time_remaining) => {
                if time_remaining <= 0 {
                    self.health = InfectionStatus::Susceptible;
                    self.mobile = true;
                    self.hospitalized = false;
                } else {
                    self.health = InfectionStatus::Recovered(time_remaining - 1);
                }
            },
            _ => {},
        }
    }
}