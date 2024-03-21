use std::time::{Duration, Instant};

pub struct Repaint {
    outstanding: usize,
    delay: Duration,
}

impl Default for Repaint {
    fn default() -> Self {
        Self::new()
    }
}

impl Repaint {
    pub const fn new() -> Self {
        Self {
            outstanding: 0,
            delay: Duration::MAX,
        }
    }

    pub fn request_repaint_after(&mut self, delay: Duration) {
        if delay == Duration::ZERO {
            self.outstanding = 1;
        }

        if delay < self.delay {
            self.delay = delay;
        }
    }

    #[allow(dead_code)]
    pub fn has_requested_repaint(&self) -> bool {
        0 < self.outstanding || self.delay < Duration::MAX
    }

    pub fn remaining(&self, clock: Instant) -> Duration {
        self.delay.saturating_sub(clock.elapsed())
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
