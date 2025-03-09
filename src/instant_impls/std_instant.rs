// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

extern crate std;

use core::time::Duration;

use crate::Instant;

impl Instant for ::std::time::Instant {
    fn now() -> Self {
        Self::now()
    }

    fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.checked_add(duration)
    }

    fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.checked_sub(duration)
    }

    fn saturating_duration_since(&self, earlier: Self) -> Duration {
        self.saturating_duration_since(earlier)
    }
}
