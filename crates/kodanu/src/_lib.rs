pub mod prelude {
    pub use kodanu_app::App;
    pub use kodanu_assets::{Material, Mesh, Vertex};
    pub use kodanu_camera::{Camera, PerspectiveProjection, Projection};
    pub use kodanu_color::Color;
    pub use kodanu_ecs::{Entity, Read, World, WorldCell, Write};
    pub use kodanu_editor::EditorView;
    pub use kodanu_graphics::{Backend, MeshRenderer, RendererConfig, SampleCount};
    pub use kodanu_input::{Action, ActionMap, Axis, Input, KeyCode, MouseKey};
    pub use kodanu_log::{Level, LogConfig};
    pub use kodanu_math::{DVec2, EulerRot, Mat4, Quat, UVec2, Vec2, Vec3, Vec4};
    pub use kodanu_plugin::{Plugin, PluginRegistry};
    pub use kodanu_scheduler::Stage;
    pub use kodanu_time::Time;
    pub use kodanu_transform::Transform;
    pub use kodanu_window::WindowConfig;

    #[cfg(feature = "physics")]
    pub use kodanu_physics::prelude::*;
    #[cfg(feature = "plugins")]
    pub use kodanu_plugins::{DemoMeshScenePlugin, FreeCameraPlugin};
}

#[cfg(feature = "physics")]
pub use kodanu_physics as physics;
#[cfg(feature = "plugins")]
pub use kodanu_plugins as plugins;
