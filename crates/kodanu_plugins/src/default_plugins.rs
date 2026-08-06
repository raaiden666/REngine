use {
    kodanu_input::InputPlugin,
    kodanu_physics::PhysicsPlugin,
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_time::TimePlugin,
};

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut PluginRegistry) {
        app.add_plugin(InputPlugin);
        app.add_plugin(TimePlugin);
        app.add_plugin(PhysicsPlugin);
    }
}
