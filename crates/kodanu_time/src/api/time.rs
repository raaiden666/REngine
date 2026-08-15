use web_time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub(crate) startup: Instant,
    pub(crate) last: Instant,
    pub(crate) delta: Duration,
    pub(crate) elapsed: Duration,
    pub(crate) max_delta: Duration,
}

impl Default for Time {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            startup: now,
            last: now,
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            max_delta: Duration::from_millis(100),
        }
    }
}

impl Time {
    #[inline]
    pub fn delta(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    #[inline]
    pub fn elapsed(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }
}
