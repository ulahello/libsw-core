// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

use core::fmt::Debug;
use core::time::Duration;

/// A trait outlining the behavior of a timekeeping type.
///
/// This trait allows `libsw` to be agnostic about timekeeping: any type which
/// implements `Instant` can be used within a
/// [`Stopwatch`](crate::Stopwatch).
///
/// # Provided implementations
///
/// `libsw_core` provides `Instant` implementations for timekeeping types in the
/// standard library.
///
/// | Type                    | Feature flag | Notes       |
/// |-------------------------|--------------|-------------|
/// | `std::time::Instant`    | `std`        |             |
/// | `std::time::SystemTime` | `std`        |             |
/// | `tokio::time::Instant`  | `tokio`      |             |
/// | `coarsetime::Instant`   | `coarsetime` |             |
/// | `quanta::Instant`       | `quanta`     |             |
/// | `time::Instant`         | `time`       | Deprecated. |
///
/// If a timekeeping type you want to use isn't supported out of the box, please
/// consider [filing an issue](https://github.com/ulahello/libsw-core/issues)
/// on GitHub. If you already implemented `Instant` for it, consider sending a
/// PR upstream.
pub trait Instant: Copy + Debug + Sized {
    /// Returns the current instant in time.
    fn now() -> Self;

    /// Returns an instant ahead of `self` by the given [`Duration`] of time.
    ///
    /// Returns [`None`] if overflow occured, meaning the new instant was not
    /// representable with the underlying type.
    fn checked_add(&self, duration: Duration) -> Option<Self>;

    /// Returns an instant previous to `self` by the given [`Duration`] of time.
    ///
    /// Returns [`None`] if overflow occured, meaning the new instant was not
    /// representable with the underlying type.
    fn checked_sub(&self, duration: Duration) -> Option<Self>;

    /// Returns the [`Duration`] that has elapsed since `earlier`, returning
    /// [`Duration::ZERO`] if `earlier` is ahead of `self`.
    fn saturating_duration_since(&self, earlier: Self) -> Duration;
}
