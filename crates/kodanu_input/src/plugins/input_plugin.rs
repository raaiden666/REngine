use crate::{ActionMap, Input};

use {
    kodanu_ecs::{WorldCell, Write},
    kodanu_math::Vec2,
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_scheduler::Stage,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut PluginRegistry) {
        app.insert_resource(Input::with_capacity(128));
        app.insert_resource(ActionMap::with_capacity(128));

        app.add_system(Stage::PostRender, update_end_frame_system);
    }
}

#[inline]
fn update_end_frame_system(world: WorldCell) {
    let input = world.res::<Write<Input>>();

    input.keyboard_mut().end_frame();
    input.mouse_mut().end_frame();

    input.set_mouse_wheel_delta(Vec2::ZERO);
}
