use crate::AppConfig;

use {
    kodanu_graphics::{RenderItem, Renderer},
    kodanu_math::{Mat4, UVec2},
    kodanu_window::Window,
};

use winit::{event_loop::ActiveEventLoop, window::WindowAttributes};

pub(crate) struct AppRuntime {
    window: Window,
    renderer: Renderer,
}

impl AppRuntime {
    pub fn new(event_loop: &dyn ActiveEventLoop, config: &AppConfig) -> Self {
        let window = event_loop
            .create_window(WindowAttributes::from(config.window()))
            .expect("Failed to create window");

        let window = Window::new(window);
        let renderer = Renderer::new(&window, config.renderer());

        window.request_redraw();

        Self { window, renderer }
    }
}

impl AppRuntime {
    pub fn render(&mut self, view_projection: Mat4, items: &[RenderItem]) {
        self.renderer.render(view_projection, items);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn surface_resize(&mut self, size: UVec2) {
        self.renderer.surface_resize(size);
    }
}
