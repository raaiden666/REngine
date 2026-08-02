use {
    kodanu_camera::{ActiveCamera, Camera},
    kodanu_ecs::{Read, WorldCell, Write},
    kodanu_math::Mat4,
    kodanu_transform::Transform,
};

#[derive(Default, Debug)]
pub struct EditorView {
    view_projection: Mat4,
}

impl EditorView {
    #[inline]
    pub fn update_view_system(world: WorldCell) {
        let editor = world.res::<Write<EditorView>>();

        for (transform, camera, _) in
            world.view::<(Read<Transform>, Read<Camera>, Read<ActiveCamera>)>()
        {
            editor.view_projection = camera.view_projection(transform);
        }
    }
}

impl EditorView {
    #[inline]
    pub fn view_projection(&self) -> Mat4 {
        self.view_projection
    }
}
