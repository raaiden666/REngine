use {crate::prelude::*, crate::rapier3d::*};

use {
    kodanu_ecs::{Read, WorldCell, Write},
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_scheduler::Stage,
    kodanu_transform::Transform,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut PluginRegistry) {
        app.insert_resource(PhysicsWorld::default());

        app.add_system(Stage::PreFixedUpdate, spawn_rigid_bodies_system);
        app.add_system(Stage::PreFixedUpdate, spawn_collider_shapes_system);
        app.add_system(Stage::PreFixedUpdate, sync_kinematic_bodies_system);

        app.add_system(Stage::FixedUpdate, update_physics_system);

        app.add_system(Stage::PostFixedUpdate, sync_dynamic_bodies_system);
    }
}

fn spawn_rigid_bodies_system(world: WorldCell) {
    let physics = world.res::<Write<PhysicsWorld>>();

    for (transform, rigid_body) in world.view::<(Read<Transform>, Write<RigidBody>)>() {
        if rigid_body.handle.is_some() {
            continue;
        }

        rigid_body.handle = Some(physics.create_rigid_body(rigid_body, transform))
    }
}

fn spawn_collider_shapes_system(world: WorldCell) {
    let physics = world.res::<Write<PhysicsWorld>>();

    for (rigidbody, collider) in world.view::<(Read<RigidBody>, Write<Collider>)>() {
        if collider.handle.is_some() {
            continue;
        }

        let Some(handle) = rigidbody.handle else {
            continue;
        };

        collider.handle = Some(physics.create_collider(collider, handle));
    }
}

fn sync_kinematic_bodies_system(world: WorldCell) {
    let physics = world.res::<Write<PhysicsWorld>>();

    for (transform, rigid_body) in world.view::<(Read<Transform>, Write<RigidBody>)>() {
        let Some(handle) = rigid_body.handle else {
            continue;
        };

        if rigid_body.body_type != RapierRigidBodyType::KinematicPositionBased {
            continue;
        }

        let Some(body) = physics.rigid_body_mut(handle) else {
            continue;
        };

        let position = RapierPose3::from_parts(transform.position(), transform.rotation());

        body.set_next_kinematic_position(position);
    }
}

fn sync_dynamic_bodies_system(world: WorldCell) {
    let physics = world.res::<Write<PhysicsWorld>>();

    for (transform, rigid_body) in world.view::<(Write<Transform>, Write<RigidBody>)>() {
        let Some(handle) = rigid_body.handle else {
            continue;
        };

        if rigid_body.body_type != RapierRigidBodyType::Dynamic {
            continue;
        }

        let Some(body) = physics.rigid_body(handle) else {
            continue;
        };

        let position = body.position();

        transform.set_position_and_rotation(position.translation, position.rotation);
    }
}

fn update_physics_system(world: WorldCell) {
    world.res::<Write<PhysicsWorld>>().step();
}
