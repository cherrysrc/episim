use crate::entity::Entity;

pub struct Hospital {
    capacity: u32,
    count: u32,
}

impl Hospital {
    pub fn new(capacity: u32) -> Hospital {
        Hospital { capacity, count: 0 }
    }

    pub fn try_hospitalize(
        &mut self,
        entity: &mut Entity,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.count < self.capacity {
            self.count += 1;
            entity.hospitalize();
            Ok(())
        } else {
            Err(format!("Hospital is full").into())
        }
    }

    pub fn dehospitalize(&mut self, entity: &mut Entity) {
        self.count -= 1;
        entity.release();
    }

    pub fn is_full(&self) -> bool {
        self.count >= self.capacity
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}
