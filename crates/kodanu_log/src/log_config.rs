use crate::Level;

use tracing_subscriber::EnvFilter;

#[derive(Default, Debug, Clone)]
pub struct LogConfig {
    filter: EnvFilter,
}

impl LogConfig {
    pub fn with_level(self, level: Level) -> Self {
        Self {
            filter: EnvFilter::new(level.as_str()),
        }
    }

    pub fn with_directive(mut self, directive: &str) -> Self {
        self.filter = self
            .filter
            .add_directive(directive.parse().expect("Invalid log directive"));
        self
    }

    pub fn env_filter(&self) -> EnvFilter {
        self.filter.clone()
    }
}
