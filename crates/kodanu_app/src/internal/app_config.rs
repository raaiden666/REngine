use {kodanu_graphics::RendererConfig, kodanu_log::LogConfig, kodanu_window::WindowConfig};

#[derive(Debug, Default)]
pub(crate) struct AppConfig {
    window_config: WindowConfig,
    renderer_config: RendererConfig,
    log_config: LogConfig,
}

impl AppConfig {
    pub fn set_window_config(&mut self, config: WindowConfig) {
        self.window_config = config;
    }

    pub fn set_renderer_config(&mut self, config: RendererConfig) {
        self.renderer_config = config;
    }

    pub fn set_log_config(&mut self, config: LogConfig) {
        self.log_config = config;
    }

    pub fn window(&self) -> &WindowConfig {
        &self.window_config
    }

    pub fn renderer(&self) -> &RendererConfig {
        &self.renderer_config
    }

    pub fn log(&self) -> &LogConfig {
        &self.log_config
    }
}
