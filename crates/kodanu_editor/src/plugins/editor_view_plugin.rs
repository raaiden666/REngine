use crate::EditorView;

use {
    kodanu_camera::{ActiveCamera, Camera},
    kodanu_ecs::{Read, WorldCell, Write},
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_scheduler::Stage,
    kodanu_transform::Transform,
};

pub struct EditorViewPlugin;

impl Plugin for EditorViewPlugin {
    fn build(&self, app: &mut PluginRegistry) {
        app.insert_resource(EditorView::default());

        app.add_system(Stage::Render, update_view_system);
    }
}

#[inline]
fn update_view_system(world: WorldCell) {
    let editor = world.res::<Write<EditorView>>();

    for (transform, camera, _) in
        world.view::<(Read<Transform>, Read<Camera>, Read<ActiveCamera>)>()
    {
        editor.set_view_projection(camera.view_projection(transform));
    }
}
