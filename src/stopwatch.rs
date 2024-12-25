// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

use ::core::hash::{Hash, Hasher};
use ::core::ops;
use ::core::time::Duration;

use crate::canonical::Canonical;
use crate::Instant;

/// A stopwatch measures and accumulates elapsed time between starts and stops.
///
/// Stopwatches work with any type that implements [`Instant`].
///
/// # Notes
///
/// It is possible to craft two stopwatches whose internal components differ,
/// but are equal according to [`PartialEq`], [`Eq`], and [`Hash`].
///
/// ```
/// # use libsw_core::Sw;
/// # use core::time::Duration;
/// # use std::time::Instant;
/// let elapsed = Duration::from_secs(10);
/// let start = Instant::now();
/// let sw_1 = Sw {
///     elapsed,
///     start: Some(start),
/// };
/// let sw_2 = Sw {
///     // `elapsed()` is 1s less
///     elapsed: elapsed - Duration::from_secs(1),
///     // now with start pushed back, `elapsed()` is equal
///     start: Some(start - Duration::from_secs(1)),
/// };
///
/// // different components, but they are equal!
/// assert_eq!(sw_1, sw_2);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Stopwatch<I: Instant> {
    /// Accumulated elapsed time.
    pub elapsed: Duration,
    /// The instant at which the stopwatch was started, if it is running.
    /// Otherwise, [`None`].
    pub start: Option<I>,
}

impl<I: Instant> Stopwatch<I> {
    /// Returns a stopped stopwatch with zero elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let sw = Sw::new();
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self::with_elapsed(Duration::ZERO)
    }

    /// Returns a running stopwatch initialized with zero elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// let sw = Sw::new_started();
    /// assert!(sw.is_running());
    /// ```
    #[must_use]
    pub fn new_started() -> Self {
        Self::with_elapsed_started(Duration::ZERO)
    }

    /// Returns a stopwatch initialized with zero elapsed time, started at the
    /// given instant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::time::Instant;
    /// let now = Instant::now();
    /// let sw_1 = Sw::new_started_at(now);
    /// let sw_2 = Sw::new_started_at(now);
    /// // they've both started at the same time
    /// assert_eq!(sw_1, sw_2);
    /// // (and had zero elapsed time when they started)
    /// assert_eq!(sw_1.elapsed_at(now), Duration::ZERO);
    /// ```
    #[must_use]
    pub const fn new_started_at(start: I) -> Self {
        Self::from_raw(Duration::ZERO, Some(start))
    }

    /// Returns a stopped stopwatch with the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let sw = Sw::with_elapsed(Duration::from_secs(1));
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    #[must_use]
    pub const fn with_elapsed(elapsed: Duration) -> Self {
        Self::from_raw(elapsed, None)
    }

    /// Returns a running stopwatch initialized with the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let sw = Sw::with_elapsed_started(Duration::from_secs(1));
    /// assert!(sw.is_running());
    /// assert!(sw.elapsed() >= Duration::from_secs(1));
    /// ```
    #[must_use]
    pub fn with_elapsed_started(elapsed: Duration) -> Self {
        Self::from_raw(elapsed, Some(I::now()))
    }

    /// Returns a stopwatch from its raw parts.
    ///
    /// See the [top-level documentation](`Stopwatch`) for more details.
    #[must_use]
    pub const fn from_raw(elapsed: Duration, start: Option<I>) -> Self {
        Self { elapsed, start }
    }

    /// Returns `true` if the stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// let sw = Sw::new_started();
    /// assert!(sw.is_running());
    /// ```
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.start.is_some()
    }

    /// Returns `true` if the stopwatch is stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// let sw = Sw::new();
    /// assert!(sw.is_stopped());
    /// ```
    #[must_use]
    pub const fn is_stopped(&self) -> bool {
        !self.is_running()
    }

    /// Returns the total time elapsed. If overflow occurs, the elapsed time is
    /// saturated to [`Duration::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let sw = Sw::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(sw.elapsed() >= Duration::from_millis(100));
    /// ```
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.elapsed_at(I::now())
    }

    /// Returns the total time elapsed, measured as if the current time were
    /// `anchor`. If overflow occurs, the elapsed time is saturated to
    /// [`Duration::MAX`].
    ///
    /// # Notes
    ///
    /// `anchor` saturates to the last instant the stopwatch was started.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use std::time::Instant;
    /// let sw_1 = Sw::new_started();
    /// let sw_2 = sw_1;
    /// let anchor = Instant::now();
    /// assert!(sw_1.elapsed_at(anchor) == sw_2.elapsed_at(anchor));
    /// ```
    #[must_use]
    pub fn elapsed_at(&self, anchor: I) -> Duration {
        self.checked_elapsed_at(anchor).unwrap_or(Duration::MAX)
    }

    /// Computes the total time elapsed. If overflow occurred, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Sw::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(sw.checked_elapsed().unwrap() >= Duration::from_millis(100));
    /// sw += Duration::MAX;
    /// assert!(sw.checked_elapsed().is_none());
    /// ```
    #[must_use]
    pub fn checked_elapsed(&self) -> Option<Duration> {
        self.checked_elapsed_at(I::now())
    }

    /// Computes the total time elapsed, measured as if the current time were
    /// `anchor`. If overflow occurred, returns [`None`].
    ///
    /// # Notes
    ///
    /// `anchor` saturates to the last instant the stopwatch was started.
    #[must_use]
    pub fn checked_elapsed_at(&self, anchor: I) -> Option<Duration> {
        let before_start = self.elapsed;
        if let Some(start) = self.start {
            let after_start = anchor.saturating_duration_since(start);
            before_start.checked_add(after_start)
        } else {
            Some(before_start)
        }
    }

    /// Starts measuring the time elapsed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Sw::new();
    /// sw.start();
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then != now);
    /// ```
    pub fn start(&mut self) {
        self.start_at(I::now());
    }

    /// Starts measuring the time elapsed as if the current time were `anchor`.
    /// If the stopwatch is already running, the prior start time is overwritten.
    ///
    /// # Notes
    ///
    /// If `anchor` is ahead of the present, [`elapsed`](Self::elapsed) will
    /// return [`Duration::ZERO`] until the current time catches up to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # use std::time::Instant;
    /// let mut sw_1 = Sw::new();
    /// let mut sw_2 = Sw::new();
    ///
    /// let start = Instant::now();
    /// // off to the races! at the same time!
    /// sw_1.start_at(start);
    /// sw_2.start_at(start);
    ///
    /// thread::sleep(Duration::from_millis(100));
    /// let anchor = Instant::now();
    ///
    /// assert_eq!(sw_1.elapsed_at(anchor), sw_2.elapsed_at(anchor)); // 'twas a tie
    /// assert!(sw_1.elapsed_at(anchor) >= Duration::from_millis(100));
    /// ```
    pub fn start_at(&mut self, anchor: I) {
        self.start = Some(anchor);
    }

    /// Stops measuring the time elapsed since the last start.
    ///
    /// # Notes
    ///
    /// Overflows of the new elapsed time are saturated to [`Duration::MAX`].
    /// Use [`Stopwatch::checked_stop`] to explicitly check for overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Sw::new_started();
    /// sw.stop();
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then == now);
    /// ```
    pub fn stop(&mut self) {
        self.stop_at(I::now());
    }

    /// Stops measuring the time elapsed since the last start as if the current
    /// time were `anchor`.
    ///
    /// # Notes
    ///
    /// - If `anchor` is earlier than the last start, there is no effect on the
    ///   elapsed time.
    ///
    /// - Overflows of the new elapsed time are saturated to [`Duration::MAX`].
    ///   Use [`Stopwatch::checked_stop_at`] to explicitly check for overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # use std::time::Instant;
    /// let mut sw_1 = Sw::new_started();
    /// let mut sw_2 = sw_1;
    /// let stop = Instant::now();
    /// sw_1.stop_at(stop);
    /// sw_2.stop_at(stop);
    /// assert_eq!(sw_1, sw_2);
    /// ```
    pub fn stop_at(&mut self, anchor: I) {
        if let Some(start) = self.start.take() {
            let after_start = anchor.saturating_duration_since(start);
            *self = self.saturating_add(after_start);
        }
    }

    /// Tries to stop the stopwatch. If the new elapsed time overflows, returns
    /// `false` without mutating the stopwatch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::new_started();
    /// assert!(sw.checked_stop());
    /// sw.set(Duration::MAX);
    /// sw.start();
    /// assert!(!sw.checked_stop());
    /// ```
    #[must_use]
    pub fn checked_stop(&mut self) -> bool {
        self.checked_stop_at(I::now())
    }

    /// Tries to stop the stopwatch, as if the current time were `anchor`. If
    /// the new elapsed time overflows, returns `false` without mutating the
    /// stopwatch.
    ///
    /// # Notes
    ///
    /// If `anchor` is earlier than the last start, there is no effect on the
    /// elapsed time.
    #[must_use]
    pub fn checked_stop_at(&mut self, anchor: I) -> bool {
        if let Some(start) = self.start {
            let after_start = anchor.saturating_duration_since(start);
            if let Some(new) = self.checked_add(after_start) {
                self.set(new.elapsed);
            } else {
                return false;
            }
        }
        true
    }

    /// Toggles whether the stopwatch is running or stopped.
    ///
    /// # Notes
    ///
    /// See [`stop`](Self::stop) for details about how overflow is handled.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// let mut sw = Sw::new();
    /// sw.toggle();
    /// assert!(sw.is_running());
    /// sw.toggle();
    /// assert!(sw.is_stopped());
    /// ```
    pub fn toggle(&mut self) {
        self.toggle_at(I::now());
    }

    /// Toggles whether the stopwatch is running or stopped, as if the current
    /// time were `anchor`.
    ///
    /// # Notes
    ///
    /// See [`start_at`](Self::start_at) and [`stop_at`](Self::stop_at) for
    /// notes about the chronology of `anchor`, as well as what happens if
    /// overflow occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use std::time::Instant;
    /// let mut left = Sw::new();
    /// let mut right = Sw::new_started();
    ///
    /// // perfect swap of left and right running
    /// let now = Instant::now();
    /// left.toggle_at(now);
    /// right.toggle_at(now);
    ///
    /// assert!(left.is_running());
    /// assert!(right.is_stopped());
    /// ```
    pub fn toggle_at(&mut self, anchor: I) {
        if self.is_running() {
            self.stop_at(anchor);
        } else {
            self.start_at(anchor);
        }
    }

    /// Tries to toggle whether the stopwatch is running or stopped. If the new
    /// elapsed time overflows, returns `false` without mutating the stopwatch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Sw::with_elapsed_started(Duration::MAX);
    /// thread::sleep(Duration::from_millis(100));
    /// // whoops, new elapsed time can't be Duration::MAX + 100ms
    /// assert!(!sw.checked_toggle());
    /// ```
    #[must_use]
    pub fn checked_toggle(&mut self) -> bool {
        self.checked_toggle_at(I::now())
    }

    /// Tries to toggle whether the stopwatch is running or stopped, as if the
    /// current time were `anchor`. If the new elapsed time overflows, returns
    /// `false` without mutating the stopwatch.
    #[must_use]
    pub fn checked_toggle_at(&mut self, anchor: I) -> bool {
        if self.is_running() {
            self.checked_stop_at(anchor)
        } else {
            self.start_at(anchor);
            true
        }
    }

    /// Stops and resets the elapsed time to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed_started(Duration::from_secs(1));
    /// sw.reset();
    /// assert_eq!(sw, Sw::new());
    /// ```
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Resets the elapsed time to zero without affecting whether the stopwatch
    /// is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed_started(Duration::from_secs(1));
    /// sw.reset_in_place();
    /// assert!(sw.is_running());
    /// // new elapsed time is close to zero
    /// assert!(sw.elapsed() < Duration::from_millis(1));
    ///
    /// sw.stop();
    /// sw.reset_in_place();
    /// assert_eq!(sw, Sw::new());
    /// ```
    pub fn reset_in_place(&mut self) {
        self.reset_in_place_at(Instant::now());
    }

    /// Resets the elapsed time to zero without affecting whether the stopwatch
    /// is running.
    ///
    /// # Notes
    ///
    /// See [`start_at`](Self::start_at) for notes about the chronology of
    /// `anchor`.
    pub fn reset_in_place_at(&mut self, start: I) {
        self.set_in_place_at(Duration::ZERO, start);
    }

    /// Stops and sets the total elapsed time to `new`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::new();
    /// sw.set(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    pub fn set(&mut self, new: Duration) {
        *self = Self::with_elapsed(new);
    }

    /// Sets the total elapsed time to `new` without affecting whether the
    /// stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::new();
    /// sw.set_in_place(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// assert!(sw.is_stopped());
    ///
    /// sw.start();
    /// sw.set_in_place(Duration::from_secs(2));
    /// assert!(sw.elapsed() >= Duration::from_secs(2));
    /// assert!(sw.is_running());
    /// ```
    pub fn set_in_place(&mut self, new: Duration) {
        self.set_in_place_at(new, Instant::now());
    }

    /// Sets the total elapsed time to `new` as if the current time were
    /// `anchor`, and without affecting whether the stopwatch is running.
    ///
    /// # Notes
    ///
    /// See [`start_at`](Self::start_at) for notes about the chronology of
    /// `anchor`.
    pub fn set_in_place_at(&mut self, new: Duration, anchor: I) {
        let was_running = self.is_running();
        self.set(new);
        if was_running {
            self.start_at(anchor);
        }
    }

    /// Stops and sets the total elapsed time to `new`, returning the previous
    /// elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed(Duration::from_secs(3));
    /// let previous = sw.replace(Duration::from_secs(1));
    /// assert_eq!(previous, Duration::from_secs(3));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    pub fn replace(&mut self, new: Duration) -> Duration {
        self.replace_at(new, Instant::now())
    }

    /// Stops and sets the total elapsed time to `new`, returning the previous
    /// elapsed time as if the current time were `anchor`.
    ///
    /// # Notes
    ///
    /// See [`elapsed_at`](Self::elapsed_at) for notes about the chronology of
    /// `anchor`.
    pub fn replace_at(&mut self, new: Duration, anchor: I) -> Duration {
        let old = self.elapsed_at(anchor);
        self.set(new);
        old
    }

    /// Adds `dur` to the total elapsed time. If overflow occurred, the total
    /// elapsed time is set to [`Duration::MAX`].
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed(Duration::from_secs(1));
    /// sw = sw.saturating_add(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(2));
    /// sw = sw.saturating_add(Duration::MAX);
    /// assert_eq!(sw.elapsed(), Duration::MAX);
    /// ```
    #[must_use]
    pub const fn saturating_add(mut self, dur: Duration) -> Self {
        self.elapsed = self.elapsed.saturating_add(dur);
        self
    }

    /// Subtracts `dur` from the total elapsed time. If underflow occurred, the
    /// total elapsed time is set to [`Duration::ZERO`].
    ///
    /// # Notes
    ///
    /// See the documentation for [`saturating_sub_at`](Self::saturating_sub_at)
    /// for notes about positive overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed(Duration::from_secs(1));
    /// sw = sw.saturating_sub(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// sw = sw.saturating_sub(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// ```
    #[must_use]
    pub fn saturating_sub(self, dur: Duration) -> Self {
        self.saturating_sub_at(dur, I::now())
    }

    /// Subtracts `dur` from the total elapsed time, as if the current time were
    /// `anchor`. If underflow occurred, the total elapsed time is set to
    /// [`Duration::ZERO`].
    ///
    /// # Notes
    ///
    /// - If the elapsed time is overflowing (as in, exceeds [`Duration::MAX`]
    ///   prior to subtraction), the elapsed time is clamped to
    ///   [`Duration::MAX`] and *then* `dur` is subtracted from that.
    ///
    /// - `anchor` saturates to the last instant the stopwatch was started.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::time::Instant;
    /// # use std::thread;
    /// let mut sw = Sw::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = Instant::now();
    /// sw = sw.saturating_sub_at(Duration::from_secs(1), now);
    /// assert_eq!(sw.elapsed_at(now), Duration::ZERO);
    /// ```
    #[must_use]
    pub fn saturating_sub_at(mut self, dur: Duration, mut anchor: I) -> Self {
        self.saturate_anchor_to_start(&mut anchor);
        self.saturating_sync_elapsed_at(anchor);
        self.elapsed = self.elapsed.saturating_sub(dur);
        self
    }

    /// Adds `dur` to the total elapsed time. If overflow occurred, returns
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::new();
    /// sw = sw.checked_add(Duration::from_secs(1)).unwrap();
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// assert_eq!(sw.checked_add(Duration::MAX), None);
    /// ```
    #[must_use]
    pub const fn checked_add(mut self, dur: Duration) -> Option<Self> {
        match self.elapsed.checked_add(dur) {
            Some(new) => {
                self.elapsed = new;
                Some(self)
            }
            None => None,
        }
    }

    /// Subtracts `dur` from the total elapsed time. If overflow occurred,
    /// returns [`None`].
    ///
    /// # Notes
    ///
    /// See the documentation for [`checked_sub_at`](Self::checked_sub_at) for
    /// notes about positive overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::new();
    /// assert_eq!(sw.checked_sub(Duration::from_secs(1)), None);
    /// sw += Duration::from_secs(1);
    /// assert_eq!(
    ///     sw.checked_sub(Duration::from_secs(1)),
    ///     Some(Sw::with_elapsed(Duration::ZERO)),
    /// );
    /// ```
    #[must_use]
    pub fn checked_sub(self, dur: Duration) -> Option<Self> {
        self.checked_sub_at(dur, I::now())
    }

    /// Subtracts `dur` from the total elapsed time, as if the current time were
    /// `anchor`. If overflow occurred, returns [`None`].
    ///
    /// # Notes
    ///
    /// - Overflow occurs if the elapsed time overflows prior to subtraction.
    ///
    /// - `anchor` saturates to the last instant the stopwatch was started.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw_core::Sw;
    /// # use core::time::Duration;
    /// # use std::time::Instant;
    /// # use std::thread;
    /// let mut sw = Sw::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = Instant::now();
    /// // underflow yields `None`
    /// assert_eq!(sw.checked_sub_at(Duration::from_secs(1), now), None);
    ///
    /// // positive overflow yields `None`
    /// sw.set_in_place(Duration::MAX);
    /// assert_eq!(sw.checked_sub(Duration::ZERO), None);
    /// assert_eq!(sw.checked_sub(Duration::from_secs(2)), None);
    /// ```
    #[must_use]
    pub fn checked_sub_at(mut self, dur: Duration, mut anchor: I) -> Option<Self> {
        self.saturate_anchor_to_start(&mut anchor);
        self.checked_sync_elapsed_at(anchor)?;
        let new = self.elapsed.checked_sub(dur)?;
        self.elapsed = new;
        Some(self)
    }
}

// private methods
impl<I: Instant> Stopwatch<I> {
    /// Clamp `anchor` such that when `start` is present, `start <= anchor`.
    fn saturate_anchor_to_start(&self, anchor: &mut I) {
        if let Some(start) = self.start {
            // Instant doesn't implement PartialOrd, so we measure their
            // difference in both directions to order them.
            // - iff `anchor` < `start`, then `past` is nonzero and `future` is zero
            // - iff `start` < `anchor`, then `future` is nonzero and `past` is zero
            // - iff `start` == `anchor`, then both `future` and `past` are zero

            let future = anchor.saturating_duration_since(start);
            let past = start.saturating_duration_since(*anchor);

            if future < past {
                *anchor = start;
            }
        }
    }

    /// Syncs changes in the elapsed time, effectively toggling the stopwatch
    /// twice. If the new elapsed time overflows, it is saturated to
    /// [`Duration::MAX`].
    fn saturating_sync_elapsed_at(&mut self, anchor: I) {
        if let Some(start) = self.start {
            *self = self.saturating_add(anchor.saturating_duration_since(start));
            self.start = Some(anchor);
        }
    }

    /// Syncs changes in the elapsed time, effectively toggling the stopwatch
    /// twice. If the new elapsed time overflows, returns [`None`] without
    /// mutating the stopwatch.
    #[must_use]
    fn checked_sync_elapsed_at(&mut self, anchor: I) -> Option<()> {
        if let Some(start) = self.start {
            let after_start = anchor.saturating_duration_since(start);
            *self = self.checked_add(after_start)?;
            self.start = Some(anchor);
        }
        Some(())
    }
}

impl<I: Instant> Default for Stopwatch<I> {
    /// Returns the default stopwatch. Same as calling [`Stopwatch::new`].
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Instant> ops::Add<Duration> for Stopwatch<I> {
    type Output = Self;

    /// Add `dur` to `self`.
    ///
    /// Currently this is an alias to [`Stopwatch::checked_add`], but that
    /// is not a stable guarentee. If you need a guarentee on the
    /// implementation, use the [checked](Self::checked_add) or
    /// [saturating](Self::checked_add) methods explicitly.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs.
    #[track_caller]
    fn add(self, dur: Duration) -> Self::Output {
        self.checked_add(dur)
            .expect("attempt to add stopwatch with overflow")
    }
}

impl<I: Instant> ops::Sub<Duration> for Stopwatch<I> {
    type Output = Self;

    /// Subtract `dur` from `self`.
    ///
    /// Currently this is an alias to [`Stopwatch::checked_sub`], but that
    /// is not a stable guarentee. If you need a guarentee on the
    /// implementation, use the [checked](Self::checked_sub) or
    /// [saturating](Self::checked_sub) methods explicitly.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs.
    #[track_caller]
    fn sub(self, dur: Duration) -> Self::Output {
        self.checked_sub(dur)
            .expect("attempt to subtract stopwatch with overflow")
    }
}

impl<I: Instant> ops::AddAssign<Duration> for Stopwatch<I> {
    #[track_caller]
    fn add_assign(&mut self, dur: Duration) {
        *self = *self + dur;
    }
}

impl<I: Instant> ops::SubAssign<Duration> for Stopwatch<I> {
    #[track_caller]
    fn sub_assign(&mut self, dur: Duration) {
        *self = *self - dur;
    }
}

impl<I: Instant> PartialEq for Stopwatch<I> {
    /// Tests for equality between `self` and `rhs`.
    ///
    /// Stopwatches are equal if whether they are running and their elapsed time
    /// are equal.
    fn eq(&self, rhs: &Self) -> bool {
        Canonical::new(*self) == Canonical::new(*rhs)
    }
}

impl<I: Instant> Eq for Stopwatch<I> {}

impl<I: Instant + Hash> Hash for Stopwatch<I> {
    /// Hashes `self` and `rhs`. These hashes are not dependent on the time of
    /// measurement, so they can be used to test equality.
    ///
    /// # Support
    ///
    /// `I` (the [`Instant`] type used by the stopwatch) must implement
    /// [`Hash`].
    fn hash<H: Hasher>(&self, state: &mut H) {
        Canonical::new(*self).hash(state);
    }
}
