use wgpu::{CurrentSurfaceTexture, Device, Surface, SurfaceConfiguration};

use kodanu_math::UVec2;

#[derive(Debug)]
pub(crate) struct RenderSurface {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    size: UVec2,
}

impl RenderSurface {
    pub fn new(surface: Surface<'static>, config: SurfaceConfiguration, size: UVec2) -> Self {
        Self {
            surface,
            config,
            size,
        }
    }
}

impl RenderSurface {
    pub fn config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn resize(&mut self, device: &Device, size: UVec2) {
        if size.x == 0 || size.y == 0 {
            return;
        }

        self.size = size;
        self.config.width = size.x;
        self.config.height = size.y;

        self.surface.configure(device, &self.config);
    }

    pub fn acquire_frame(&self) -> CurrentSurfaceTexture {
        self.surface.get_current_texture()
    }
}
