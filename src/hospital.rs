use crate::{entity::Entity, CONFIG};

pub struct Hospital {
    capacity: usize,
    enlisted: Vec<Patient>,
}

pub struct Patient {
    entity_id: usize,
    time_to_recover: u32,
}

impl Patient {
    pub fn new(entity_id: usize, time_to_recover: u32) -> Patient {
        Patient {
            entity_id,
            time_to_recover,
        }
    }

    pub fn tick(&mut self) {
        self.time_to_recover -= 1;
    }

    pub fn ready_to_release(&self) -> bool {
        self.time_to_recover <= 0
    }
}

impl Hospital {
    pub fn new(capacity: usize) -> Hospital {
        Hospital {
            capacity,
            enlisted: Vec::with_capacity(capacity as usize),
        }
    }

    pub fn try_hospitalize(
        &mut self,
        entity: &mut Entity,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_full() && !self.contains(entity) {
            self.enlisted.push(Patient::new(
                entity as *const _ as usize,
                CONFIG.core.hospital_period,
            ));
            entity.set_hospitalized();
            Ok(())
        } else {
            Err(format!("Hospital is full").into())
        }
    }

    pub fn contains(&self, entity: &Entity) -> bool {
        self.enlisted
            .iter()
            .any(|patient| patient.entity_id == entity as *const _ as usize)
    }

    pub fn count(&self) -> usize {
        self.enlisted.len()
    }

    pub fn is_full(&self) -> bool {
        self.enlisted.len() >= self.capacity
    }

    pub fn update(&mut self) {
        for patient in &mut self.enlisted {
            patient.tick();
        }
    }

    pub fn release(&mut self, entity: &mut Entity) {
        let mut index = 0;
        for patient in &mut self.enlisted {
            if patient.entity_id == entity as *const _ as usize {
                self.enlisted.remove(index);
                entity.unset_hospitalized();
                break;
            }
            index += 1;
        }
    }

    pub fn ready_to_release(&self, entity: &Entity) -> bool {
        self.enlisted
            .iter()
            .find(|p| p.entity_id == entity as *const _ as usize)
            .map(|p| p.ready_to_release())
            .unwrap_or(false)
    }
}
