// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

extern crate coarsetime;

use ::core::time::Duration;

use crate::Instant;

/* TODO: coarsetime::Instant uses coarsetime::Duration but this lib uses
 * core::time::Duration. this may create friction in the api. */

impl Instant for coarsetime::Instant {
    #[inline]
    fn now() -> Self {
        Self::now()
    }

    fn checked_add(&self, duration: Duration) -> Option<Self> {
        let coarse_dur = coarsetime::Duration::from(duration);
        coarsetime::Instant::checked_add(*self, coarse_dur)
    }

    fn checked_sub(&self, duration: Duration) -> Option<Self> {
        let coarse_dur = coarsetime::Duration::from(duration);
        coarsetime::Instant::checked_sub(*self, coarse_dur)
    }

    #[inline]
    fn saturating_duration_since(&self, earlier: Self) -> Duration {
        self.duration_since(earlier).into()
    }
}
