use {
    kodanu_camera::Camera,
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

        for (transform, camera) in world.view::<(Read<Transform>, Read<Camera>)>() {
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
