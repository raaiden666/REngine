use crate::{ButtonState, KeyCode, MouseKey};

use {
    kodanu_ecs::{WorldCell, Write},
    kodanu_math::{DVec2, Vec2},
};

#[derive(Default)]
pub struct Input {
    keyboard: ButtonState<KeyCode>,
    mouse: ButtonState<MouseKey>,
    mouse_position: DVec2,
    mouse_wheel_delta: Vec2,
}

impl Input {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keyboard: ButtonState::with_capacity(capacity),
            mouse: ButtonState::with_capacity(capacity),
            mouse_position: DVec2::ZERO,
            mouse_wheel_delta: Vec2::ZERO,
        }
    }
}

impl Input {
    #[inline]
    pub fn update_end_frame_system(world: WorldCell) {
        let input = world.res::<Write<Input>>();

        input.keyboard.end_frame();
        input.mouse.end_frame();

        input.mouse_wheel_delta = Vec2::ZERO;
    }
}

impl Input {
    #[inline]
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_pressed(key)
    }

    #[inline]
    pub fn key_just_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_pressed(key)
    }

    #[inline]
    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_released(key)
    }

    #[inline]
    pub fn button_pressed(&self, button: MouseKey) -> bool {
        self.mouse.is_pressed(button)
    }

    #[inline]
    pub fn button_just_pressed(&self, button: MouseKey) -> bool {
        self.mouse.is_just_pressed(button)
    }

    #[inline]
    pub fn button_just_released(&self, button: MouseKey) -> bool {
        self.mouse.is_just_released(button)
    }
}

impl Input {
    #[inline]
    pub fn set_mouse_position(&mut self, position: DVec2) {
        self.mouse_position = position;
    }

    #[inline]
    pub fn mouse_position(&self) -> DVec2 {
        self.mouse_position
    }

    #[inline]
    pub fn add_mouse_wheel_delta(&mut self, x: f32, y: f32) {
        self.mouse_wheel_delta += Vec2::new(x, y);
    }

    #[inline]
    pub fn mouse_wheel_delta(&self) -> Vec2 {
        self.mouse_wheel_delta
    }
}

impl Input {
    #[inline]
    pub(crate) fn press_key(&mut self, key: KeyCode) {
        self.keyboard.press(key);
    }

    #[inline]
    pub(crate) fn release_key(&mut self, key: KeyCode) {
        self.keyboard.release(key);
    }

    #[inline]
    pub(crate) fn press_mouse_button(&mut self, button: MouseKey) {
        self.mouse.press(button);
    }

    #[inline]
    pub(crate) fn release_mouse_button(&mut self, button: MouseKey) {
        self.mouse.release(button);
    }
}
