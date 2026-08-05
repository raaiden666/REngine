use crate::{FixedRunner, Schedule, Stage, System};

use {kodanu_ecs::WorldCell, std::array::from_fn};

pub struct Scheduler {
    schedules: [Schedule; Stage::COUNT],
    fixed_runner: FixedRunner,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            schedules: from_fn(|_| Schedule::default()),
            fixed_runner: FixedRunner::default(),
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

    #[inline]
    pub fn run_startup(&mut self, world: WorldCell) {
        self.run(Stage::Startup, world);
    }

    #[inline]
    pub fn run_fixed(&mut self, world: WorldCell, delta: f32) {
        let fixed_steps = self.fixed_runner.consume(delta);

        for _ in 0..fixed_steps {
            self.run(Stage::PreFixedUpdate, world);
            self.run(Stage::FixedUpdate, world);
            self.run(Stage::PostFixedUpdate, world);
        }
    }

    #[inline]
    pub fn run_update(&mut self, world: WorldCell) {
        self.run(Stage::PreUpdate, world);
        self.run(Stage::Update, world);
        self.run(Stage::LateUpdate, world);
    }

    #[inline]
    pub fn run_render(&mut self, world: WorldCell) {
        self.run(Stage::PreRender, world);
        self.run(Stage::Render, world);
        self.run(Stage::PostRender, world);
    }
}
