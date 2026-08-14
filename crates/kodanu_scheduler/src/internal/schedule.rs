use crate::System;

use kodanu_ecs::WorldCell;

#[derive(Default)]
pub(crate) struct Schedule {
    systems: Vec<System>,
}

impl Schedule {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            systems: Vec::with_capacity(capacity),
        }
    }
}

impl Schedule {
    pub fn add(&mut self, system: System) {
        self.systems.push(system);
    }

    #[inline]
    pub fn run(&mut self, world: WorldCell) {
        for system in &mut self.systems {
            system(world);
        }
    }
}
