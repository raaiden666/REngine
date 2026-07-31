use crate::prelude::{Collider, RigidBody};

use {
    kodanu_app::Plugin,
    kodanu_assets::{Material, Mesh},
    kodanu_color::Color,
    kodanu_ecs::WorldCell,
    kodanu_math::Vec3,
    kodanu_scene::MeshRenderer,
    kodanu_scheduler::Stage,
    kodanu_transform::Transform,
};

pub struct DemoTestPhysicsPlugin;

impl Plugin for DemoTestPhysicsPlugin {
    fn build(&self, app: &mut kodanu_app::AppBuilder) {
        app.add_system(Stage::Startup, spawn_mesh_physics_system);
    }
}

fn spawn_mesh_physics_system(world: WorldCell) {
    world.spawn_bundle((
        Transform::from_position(Vec3::new(0.0, -5.0, -7.3)),
        RigidBody::kinematic(),
        Collider::sphere(1.0f32),
        MeshRenderer::new(Mesh::triangle_2d(), Material::new(Color::BLUE)),
    ));

    world.spawn_bundle((
        Transform::from_position(Vec3::new(-0.6, 8.0, -7.5)),
        RigidBody::dynamic(),
        Collider::cube(Vec3::new(1.0, 1.0, 1.0)),
        MeshRenderer::default(),
    ));

    world.spawn_bundle((
        Transform::from_position(Vec3::new(0.6, 8.0, -7.5)),
        RigidBody::dynamic(),
        Collider::cube(Vec3::new(1.0, 1.0, 1.0)),
        MeshRenderer::default(),
    ));
}
