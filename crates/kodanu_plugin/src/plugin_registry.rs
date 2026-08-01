use {
    kodanu_ecs::{Resource, WorldCell},
    kodanu_scheduler::{Scheduler, Stage, System},
};

pub struct PluginRegistry<'a> {
    scheduler: &'a mut Scheduler,
    world: &'a mut WorldCell<'a>,
}

impl<'a> PluginRegistry<'a> {
    pub fn new(scheduler: &'a mut Scheduler, world: &'a mut WorldCell<'a>) -> Self {
        Self { scheduler, world }
    }
}

impl PluginRegistry<'_> {
    pub fn add_system(&mut self, stage: Stage, system: System) {
        self.scheduler.add(stage, system);
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> Option<R> {
        self.world.insert_resource(resource)
    }
}
