use crate::{Schedule, Stage, schedule::System};

use {kodanu_ecs::WorldCell, std::array::from_fn};

pub struct Scheduler {
    schedules: [Schedule; Stage::COUNT],
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            schedules: from_fn(|_| Schedule::default()),
        }
    }
}

impl Scheduler {
    pub fn add(&mut self, stage: Stage, system: System) {
        self.schedules[stage.as_usize()].add(system);
    }

    #[inline]
    pub fn run(&mut self, stage: Stage, world: WorldCell) {
        self.schedules[stage.as_usize()].run(world);
    }
}
