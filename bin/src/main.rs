use kodanu::prelude::*;

fn main() {
    let mut app = App::default();

    app.add_plugin(DefaultPlugins)
        .add_plugin(FreeCameraPlugin)
        .add_plugin(DemoMeshScenePlugin);

    app.run();
}
