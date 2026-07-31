use kodanu::prelude::*;

fn main() {
    let window_config = WindowConfig::default().with_title("Kodanu");

    let renderer_config = RendererConfig::default()
        .with_backends(Backend::VULKAN | Backend::DX12 | Backend::METAL)
        .with_sample_count(SampleCount::Quad);

    let log_config = LogConfig::default()
        .with_directive("wgpu_hal=error")
        .with_directive("calloop=off");

    let mut app = App::default()
        .with_window_config(window_config)
        .with_renderer_config(renderer_config)
        .with_log_config(log_config);

    app.add_plugin(DemoCameraPlugin)
        .add_plugin(DemoMeshPlugin)
        .add_plugin(DemoTestPhysicsPlugin)
        .add_plugin(PhysicsPlugin);

    app.run();
}
