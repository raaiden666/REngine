use crate::Plugin;

use {
    hashbrown::HashSet,
    kodanu_ecs::{Resource, WorldCell},
    kodanu_scheduler::{Scheduler, Stage, System},
    std::any::TypeId,
};

pub struct PluginRegistry<'a> {
    scheduler: &'a mut Scheduler,
    world: &'a mut WorldCell<'a>,
    plugins: &'a mut HashSet<TypeId>,
}

impl<'a> PluginRegistry<'a> {
    pub fn new(
        scheduler: &'a mut Scheduler,
        world: &'a mut WorldCell<'a>,
        plugins: &'a mut HashSet<TypeId>,
    ) -> Self {
        Self {
            scheduler,
            world,
            plugins,
        }
    }
}

impl PluginRegistry<'_> {
    pub fn add_system(&mut self, stage: Stage, system: System) {
        self.scheduler.add(stage, system);
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> Option<R> {
        self.world.insert_resource(resource)
    }

    pub fn add_plugin<P>(&mut self, plugin: P)
    where
        P: Plugin,
    {
        if !self.plugins.insert(TypeId::of::<P>()) {
            return;
        }

        plugin.dependencies(self);
        plugin.build(self);
    }
}
