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

pub struct DemoMeshScenePlugin;

impl Plugin for DemoMeshScenePlugin {
    fn build(&self, app: &mut kodanu_app::AppBuilder) {
        app.add_system(Stage::Startup, test_mesh_system);
    }
}

fn test_mesh_system(world: WorldCell) {
    world.spawn_bundle((
        Transform::from_position(Vec3::new(0.0, 0.0, -5.0)),
        MeshRenderer::new(Mesh::cube_2d(), Material::new(Color::GREEN)),
    ));

    world.spawn_bundle((
        Transform::from_position(Vec3::new(-2.0, 0.0, -2.5)),
        MeshRenderer::new(Mesh::cube_2d(), Material::new(Color::BLUE)),
    ));

    world.spawn_bundle((
        Transform::from_position(Vec3::new(2.0, 0.0, -2.5)),
        MeshRenderer::new(Mesh::triangle_2d(), Material::new(Color::RED)),
    ));
}
