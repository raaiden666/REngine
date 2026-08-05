use crate::Time;

use {
    kodanu_ecs::{WorldCell, Write},
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_scheduler::Stage,
    std::time::Instant,
};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut PluginRegistry) {
        app.insert_resource(Time::default());

        app.add_system(Stage::LateUpdate, update_time_system);
    }
}

#[inline]
pub fn update_time_system(world: WorldCell) {
    let time = world.res::<Write<Time>>();

    let now = Instant::now();
    let delta = now.duration_since(time.last());

    time.set_delta(delta.min(time.max_delta()));
    time.set_elapsed(now.duration_since(time.startup()));

    time.set_last(now);
}
