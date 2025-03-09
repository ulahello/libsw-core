// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

extern crate time;

use core::time::Duration;

use crate::Instant;

impl Instant for time::Instant {
    fn now() -> Self {
        Self::now()
    }

    fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.0.checked_add(duration).map(time::Instant)
    }

    fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.0.checked_sub(duration).map(time::Instant)
    }

    fn saturating_duration_since(&self, earlier: Self) -> Duration {
        self.0.saturating_duration_since(earlier.0)
    }
}
