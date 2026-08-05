use {
    kodanu_editor::EditorViewPlugin,
    kodanu_graphics::RenderQueuePlugin,
    kodanu_input::InputPlugin,
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_time::TimePlugin,
};

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut PluginRegistry) {
        app.add_plugin(InputPlugin);
        app.add_plugin(TimePlugin);
        app.add_plugin(RenderQueuePlugin);
        app.add_plugin(EditorViewPlugin);
    }
}
