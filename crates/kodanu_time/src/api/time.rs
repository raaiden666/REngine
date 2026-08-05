use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct Time {
    startup: Instant,
    last: Instant,
    delta: Duration,
    elapsed: Duration,
    max_delta: Duration,
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

impl Time {
    #[inline]
    pub(crate) fn last(&self) -> Instant {
        self.last
    }

    #[inline]
    pub(crate) fn max_delta(&self) -> Duration {
        self.max_delta
    }

    #[inline]
    pub(crate) fn startup(&self) -> Instant {
        self.startup
    }

    #[inline]
    pub(crate) fn set_delta(&mut self, delta: Duration) {
        self.delta = delta;
    }

    #[inline]
    pub(crate) fn set_elapsed(&mut self, elapsed: Duration) {
        self.elapsed = elapsed;
    }

    #[inline]
    pub(crate) fn set_last(&mut self, last: Instant) {
        self.last = last;
    }
}
