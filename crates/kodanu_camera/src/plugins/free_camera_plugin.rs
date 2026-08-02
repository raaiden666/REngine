use crate::{ActiveCamera, Camera};

use {
    kodanu_ecs::{Read, WorldCell, Write},
    kodanu_input::{ActionMap, Axis, Input},
    kodanu_math::Vec3,
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_scheduler::Stage,
    kodanu_time::Time,
    kodanu_transform::Transform,
};

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut PluginRegistry) {
        app.add_system(Stage::Startup, test_camera_system);
        app.add_system(Stage::LateUpdate, perspective_camera_system);
    }
}

fn test_camera_system(world: WorldCell) {
    world.spawn_bundle((
        Transform::default(),
        Camera::default(),
        ActiveCamera::default(),
    ));
}

fn perspective_camera_system(world: WorldCell) {
    let (input, action_map, time) = (
        world.res::<Read<Input>>(),
        world.res::<Read<ActionMap>>(),
        world.res::<Read<Time>>(),
    );

    for (transform, _, _) in world.view::<(Write<Transform>, Read<Camera>, Read<ActiveCamera>)>() {
        let direction = transform.forward() * action_map.axis(Axis::MoveY, input)
            + -transform.right() * action_map.axis(Axis::MoveX, input)
            + transform.up() * action_map.axis(Axis::MoveZ, input);

        let yaw = action_map.axis(Axis::LookX, input) * 2.0 * time.delta();
        let pitch = action_map.axis(Axis::LookY, input) * 2.0 * time.delta();

        transform.translate(direction * 10.0 * time.delta());

        transform.rotate(Vec3::Y, yaw);
        transform.rotate_local(Vec3::X, pitch);
    }
}
