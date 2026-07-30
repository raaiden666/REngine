#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Level {
    #[default]
    Info,
    Trace,
    Debug,
    Warn,
    Error,
}

impl Level {
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Trace => "trace",
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
        }
    }
}
