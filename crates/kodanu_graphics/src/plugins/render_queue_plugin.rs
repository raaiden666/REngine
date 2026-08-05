use crate::{MeshRenderer, RenderItem, RenderQueue};

use {
    kodanu_ecs::{Read, WorldCell, Write},
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_scheduler::Stage,
    kodanu_transform::Transform,
};

pub struct RenderQueuePlugin;

impl Plugin for RenderQueuePlugin {
    fn build(&self, app: &mut PluginRegistry) {
        app.insert_resource(RenderQueue::with_capacity(1_000));

        app.add_system(Stage::PreRender, update_queue_system);
    }
}

#[inline]
fn update_queue_system(world: WorldCell) {
    let queue = world.res::<Write<RenderQueue>>();

    queue.clear();

    for (transform, mesh_renderer) in world.view::<(Read<Transform>, Read<MeshRenderer>)>() {
        queue.push(RenderItem::new(
            mesh_renderer.mesh_handle(),
            mesh_renderer.material_handle(),
            transform.matrix(),
        ));
    }
}
