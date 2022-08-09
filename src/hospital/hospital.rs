use crate::{entity::Entity, CONFIG};

use super::Patient;

/// Used to enlist and release entitys to the hospital.
pub struct Hospital {
    capacity: usize,
    enlisted: Vec<Patient>,
}

impl Hospital {
    pub fn new(capacity: usize) -> Hospital {
        Hospital {
            capacity,
            enlisted: Vec::with_capacity(capacity as usize),
        }
    }

    /// Tries enlisting the entity into the hospital.
    /// Fails if the hospital is full.
    /// Sets the corresponding flag on the entity.
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

    /// Returns true if the given entity is already in the hospital.
    pub fn contains(&self, entity: &Entity) -> bool {
        self.enlisted
            .iter()
            .any(|patient| patient.entity_id == entity as *const _ as usize)
    }

    /// Returns the current count of entities in the hospital.
    pub fn count(&self) -> usize {
        self.enlisted.len()
    }

    /// Returns true if the hospital is full.
    pub fn is_full(&self) -> bool {
        self.enlisted.len() >= self.capacity
    }

    /// Counts each entitys hospitalized-timer down
    pub fn update(&mut self) {
        for patient in &mut self.enlisted {
            patient.tick();
        }
    }

    /// Releases the given entity from the hospital.
    /// Sets the corresponding flag on the entity.
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

    /// Returns true if the given entity is ready to be released
    /// aka its timer has run out.
    pub fn ready_to_release(&self, entity: &Entity) -> bool {
        self.enlisted
            .iter()
            .find(|p| p.entity_id == entity as *const _ as usize)
            .map(|p| p.ready_to_release())
            .unwrap_or(false)
    }
}
