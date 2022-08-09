/// Struct representing a patient enlisted in the hospital
/// Uses raw pointers converted to usize to identify the entity.
pub struct Patient {
    pub(super) entity_id: usize,
    pub(super) time_to_recover: u32,
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
