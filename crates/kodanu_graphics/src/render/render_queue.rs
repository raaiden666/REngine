use crate::{MeshRenderer, RenderItem};

use {kodanu_ecs::Read, kodanu_ecs::WorldCell, kodanu_ecs::Write, kodanu_transform::Transform};

#[derive(Default)]
pub struct RenderQueue {
    items: Vec<RenderItem>,
}

impl RenderQueue {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }
}

impl RenderQueue {
    #[inline]
    pub fn update_queue_system(world: WorldCell) {
        let queue = world.res::<Write<RenderQueue>>();
        let query = world.view::<(Read<Transform>, Read<MeshRenderer>)>();

        queue.items.clear();

        for (transform, mesh_renderer) in query {
            queue.items.push(RenderItem::new(
                mesh_renderer.mesh_handle(),
                mesh_renderer.material_handle(),
                transform.matrix(),
            ));
        }
    }
}

impl RenderQueue {
    #[inline]
    pub fn items(&self) -> &[RenderItem] {
        &self.items
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.items.clear();
    }
}
