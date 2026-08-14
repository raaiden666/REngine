use {
    kodanu_math::UVec2,
    std::sync::Arc,
    wgpu::{Instance, Surface},
};

use winit::window::Window as WinitWindow;

pub struct Window {
    window: Arc<dyn WinitWindow>,
}

impl Window {
    pub fn new(window: Box<dyn WinitWindow>) -> Self {
        Self {
            window: Arc::from(window),
        }
    }
}

impl Window {
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn size(&self) -> UVec2 {
        let size = self.window.surface_size();

        UVec2::new(size.height, size.width)
    }

    pub fn create_surface(&self, instance: &Instance) -> Surface<'static> {
        instance
            .create_surface(Arc::clone(&self.window))
            .expect("Failed to create surface")
    }
}
