// libsw: stopwatch library (tests)
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

/* TODOO: not designed for approximate time but coarsetime is
 * supported. it fails some of these tests; grep for @depends-exact */

/* TODO: re-organize tests */
/* TODO: Instant::checked_add is not covered at all by tests and is not used in
 * crate */

use ::core::hash::{Hash, Hasher};
use ::core::time::Duration;
use ::std::collections::hash_map::DefaultHasher;
use ::std::thread;

use crate::Instant;

/* TODO: manually changing these aliases if i want to test all supported
 * `Instant` impls is annoying */
type I = std::time::Instant;
type Stopwatch = crate::stopwatch::Stopwatch<I>;

const DELAY: Duration = Duration::from_millis(100);

#[test]
fn default() {
    assert_eq!(Stopwatch::default(), Stopwatch::new());
}

#[test]
fn new() {
    let now = I::now();
    assert_eq!(Stopwatch::new().elapsed(), Duration::ZERO);
    assert_eq!(Stopwatch::new_started().elapsed, Duration::ZERO);
    assert_eq!(
        Stopwatch::new_started_at(now).elapsed_at(now),
        Duration::ZERO
    );
}

#[test]
fn is_running() {
    let mut sw = Stopwatch::new();
    assert!(!sw.is_running());

    sw.start();
    assert!(sw.is_running());

    sw.stop();
    assert!(!sw.is_running());
}

#[test]
fn is_stopped() {
    let mut sw = Stopwatch::new();
    assert!(sw.is_stopped());

    sw.start();
    assert!(!sw.is_stopped());

    sw.stop();
    assert!(sw.is_stopped());
}

#[test]
fn toggle() {
    let mut sw = Stopwatch::new();
    assert!(sw.is_stopped());

    sw.toggle();
    assert!(sw.is_running());

    sw.toggle();
    assert!(sw.is_stopped());
}

#[test]
fn checked_toggle() {
    let mut sw = Stopwatch::new();
    assert!(sw.is_stopped());

    assert!(sw.checked_toggle());
    assert!(sw.is_running());

    assert!(sw.checked_toggle());
    assert!(sw.is_stopped());
}

#[test]
fn reset() {
    let mut sw = Stopwatch::new_started();
    thread::sleep(DELAY);

    sw.stop();
    sw.start();
    sw.reset();

    assert_eq!(sw, Stopwatch::new());
}

#[test]
fn set() {
    let mut sw = Stopwatch::new_started();
    sw.set(DELAY);
    assert_eq!(sw, Stopwatch::with_elapsed(DELAY));
}

#[test]
fn set_in_place() {
    let mut sw = Stopwatch::new_started();
    sw.set_in_place(DELAY);
    assert!(sw.is_running());
    assert!(sw.elapsed() >= DELAY);

    thread::sleep(DELAY);

    sw.set_in_place(DELAY);
    assert!(sw.is_running());
    assert!(sw.elapsed() < DELAY * 2);
}

#[test]
fn replace() {
    let mut sw = Stopwatch::with_elapsed_started(DELAY);
    let prev = sw.replace(DELAY * 2);

    assert!(sw.is_stopped());
    assert!(prev >= DELAY);
    assert_eq!(sw.elapsed(), DELAY * 2);
}

#[test]
fn add() {
    let mut sw = Stopwatch::new();

    sw += DELAY;
    sw.start();
    sw += DELAY;
    sw.stop();
    sw += DELAY;

    assert!(sw.elapsed() >= DELAY * 3);
}

#[test]
fn sub() {
    assert_eq!(
        Stopwatch::with_elapsed(DELAY * 3) - DELAY,
        Stopwatch::with_elapsed(DELAY * 2)
    );
}

#[test]
fn sub_at() {
    let mut sw = Stopwatch::with_elapsed_started(DELAY * 3);
    thread::sleep(DELAY);
    let now = I::now();
    let old_elapsed = sw.elapsed_at(now);
    sw = sw.saturating_sub_at(DELAY * 3, now);
    thread::sleep(DELAY);
    assert_eq!(sw.elapsed_at(now), old_elapsed - DELAY * 3);
}

#[test]
#[should_panic]
fn add_overloaded_overflow() {
    _ = Stopwatch::with_elapsed(Duration::MAX) + DELAY;
}

#[test]
#[should_panic]
fn sub_overloaded_overflow() {
    _ = Stopwatch::new() - DELAY;
}

#[test]
fn checked_add() {
    let mut sw = Stopwatch::new();

    sw = sw.checked_add(DELAY).unwrap();
    sw.start();
    sw = sw.checked_add(DELAY).unwrap();
    sw.stop();
    sw = sw.checked_add(DELAY).unwrap();

    assert!(sw.elapsed() >= DELAY * 3);
}

#[test]
fn checked_sub() {
    assert_eq!(
        Stopwatch::with_elapsed(DELAY * 3)
            .checked_sub(DELAY)
            .unwrap(),
        Stopwatch::with_elapsed(DELAY * 2)
    );

    let start = Some(I::now());
    assert_eq!(
        Stopwatch::from_raw(DELAY * 3, start)
            .checked_sub(DELAY)
            .unwrap(),
        Stopwatch::from_raw(DELAY * 2, start)
    );
}

#[test]
fn checked_add_overflow() {
    assert_eq!(
        Stopwatch::new().checked_add(Duration::MAX).unwrap(),
        Stopwatch::with_elapsed(Duration::MAX),
    );
    assert_eq!(
        Stopwatch::with_elapsed(DELAY).checked_add(Duration::MAX),
        None,
    );
}

#[test]
fn checked_sub_overflow() {
    assert_eq!(
        Stopwatch::with_elapsed(Duration::MAX)
            .checked_sub(Duration::MAX)
            .unwrap(),
        Stopwatch::new(),
    );
    assert_eq!(Stopwatch::with_elapsed(DELAY).checked_sub(DELAY * 2), None);
}

// @depends-exact
#[test]
fn sane_elapsed_while_stopped() {
    let mut sw = Stopwatch::new_started();
    thread::sleep(DELAY);
    sw.stop();

    assert!(sw.elapsed() >= DELAY);
}

// @depends-exact
#[test]
fn sane_elapsed_while_running() {
    let sw = Stopwatch::new_started();
    thread::sleep(DELAY);

    assert!(sw.elapsed() >= DELAY);
}

#[test]
fn sync_before_sub_saturating() {
    let mut sw = Stopwatch::new_started();
    thread::sleep(DELAY);
    sw = sw.saturating_sub(DELAY);
    assert!(sw.elapsed() < DELAY);
}

#[test]
fn sync_before_sub_checked() {
    let mut sw = Stopwatch::new_started();
    thread::sleep(DELAY);
    sw = sw.checked_sub(DELAY).unwrap(); // @depends-exact
    assert!(sw.elapsed() < DELAY);
}

#[test]
fn sync_before_sub_checked_overflow() {
    let sw = Stopwatch::with_elapsed_started(Duration::MAX);
    thread::sleep(DELAY);
    assert_eq!(sw.checked_sub(DELAY * 2), None);
}

#[test]
fn sub_at_earlier_anchor_behavior() {
    let mut sw = Stopwatch::new();

    let earlier = I::now();
    thread::sleep(DELAY);
    let later = I::now();
    thread::sleep(DELAY);

    sw.start_at(later);

    for dur in (0..10).map(Duration::from_secs) {
        assert_eq!(
            sw.checked_sub_at(dur, earlier),
            sw.checked_sub_at(dur, later)
        );
        assert!(sw.is_running());
        assert_eq!(
            sw.saturating_sub_at(dur, earlier),
            sw.saturating_sub_at(dur, later)
        );
        assert!(sw.is_running());
    }
}

#[test]
fn elapsed_at_saturates() {
    let sw = Stopwatch::with_elapsed_started(DELAY);
    let now = I::now();
    assert_eq!(
        sw.elapsed_at(Instant::checked_sub(&now, DELAY * 2).unwrap()),
        DELAY
    );
}

#[test]
fn checked_elapsed_overflows() {
    let sw = Stopwatch::with_elapsed_started(Duration::MAX);
    thread::sleep(DELAY);
    assert_eq!(sw.checked_elapsed(), None);
}

#[test]
fn start_in_future() {
    let mut sw = Stopwatch::new();
    let now = I::now();
    sw.start_at(Instant::checked_add(&now, DELAY * 2).unwrap());

    thread::sleep(DELAY);
    sw.stop();
    assert_eq!(sw.elapsed(), Duration::ZERO);
}

#[test]
fn stop_before_last_start() {
    let mut sw = Stopwatch::with_elapsed(DELAY);
    let start = I::now();
    let old_elapsed = sw.elapsed();

    sw.start_at(start);
    thread::sleep(DELAY);
    sw.stop_at(Instant::checked_sub(&start, DELAY).unwrap());

    assert_eq!(old_elapsed, sw.elapsed());
}

// @depends-exact
#[test]
fn repeat_start() {
    let mut sw = Stopwatch::with_elapsed(DELAY);
    let anchor1 = I::now();
    let anchor2 = Instant::checked_add(&anchor1, DELAY).unwrap();
    sw.start_at(anchor1);
    assert!(sw.is_running());
    assert_eq!(sw.elapsed_at(anchor2), DELAY * 2);
    sw.start_at(anchor2);
    assert_eq!(sw.elapsed_at(anchor2), DELAY);
    assert!(sw.is_running());
}

// @depends-exact
#[test]
fn repeat_stop() {
    let mut sw = Stopwatch::with_elapsed(DELAY);
    let anchor1 = I::now();
    let anchor2 = Instant::checked_add(&anchor1, DELAY).unwrap();
    sw.start_at(anchor1);
    for _ in 0..2 {
        sw.stop_at(anchor2);
        assert!(sw.is_stopped());
        assert_eq!(sw.elapsed(), DELAY * 2);
    }
}

#[test]
fn checked_stop_overflows() {
    let mut sw = Stopwatch::with_elapsed_started(Duration::MAX);
    thread::sleep(DELAY);
    assert!(sw.checked_elapsed().is_none());
    assert!(!sw.checked_stop());
    assert!(sw.is_running());
    sw.stop();
    assert!(sw.checked_stop()); // no overflow, not running
}

#[test]
fn checked_stop_stops() {
    let mut sw = Stopwatch::new_started();
    assert!(sw.is_running());
    assert!(sw.checked_stop());
    assert!(sw.is_stopped());
    assert!(sw.checked_stop()); // no overflow, not running
}

#[test]
fn eq_properties() {
    for [a, b, c] in mixed_stopwatches() {
        dbg!(a, b, c);

        // reflexive
        assert!(a == a);
        assert!(b == b);

        // symmetric
        assert_eq!(a == b, b == a);

        // transitive
        if (a == b) && (b == c) {
            assert_eq!(a, c);
        }
    }
}

#[test]
fn eq_running() {
    // whatever is compared shouldn't depend on the time of observation
    let now = I::now();
    let sw_1 = Stopwatch::new_started_at(now);
    let sw_2 = Stopwatch::new_started_at(now);
    let sw_3 = Stopwatch::from_raw(DELAY, Some(now));
    assert_eq!(sw_1, sw_2);
    assert_ne!(sw_1, sw_3);
}

#[test]
fn eq_correct() {
    assert_ne!(Stopwatch::new(), Stopwatch::new_started());
    assert_ne!(
        Stopwatch::with_elapsed(Duration::from_secs(1)),
        Stopwatch::with_elapsed(Duration::from_secs(2)),
    );

    let mut sw_1 = Stopwatch::new();
    let mut sw_2 = Stopwatch::new();
    let start = I::now();
    sw_1.start_at(start);
    sw_2.start_at(start);
    assert_eq!(sw_1, sw_2);
}

#[test]
fn partial_eq_use_of_unnormalized_instant() {
    /// Returns an instant that is pretty close to the epoch, and a duration reaching past it.
    fn approximately_the_epoch(search_start: I) -> (I, Duration) {
        let mut dt = Duration::MAX;
        let mut t = search_start;
        while Instant::checked_sub(&t, dt).is_some() || Instant::checked_add(&t, dt).is_none() {
            while let Some(new_t) = Instant::checked_sub(&t, dt) {
                t = new_t;
            }
            dt /= 2;
        }
        (t, dt)
    }

    let (anchor1, dt) = approximately_the_epoch(I::now());
    let anchor2 = Instant::checked_add(&anchor1, dt).unwrap();
    assert_eq!(None, Instant::checked_sub(&anchor1, dt));
    assert_eq!(anchor1, Instant::checked_sub(&anchor2, dt).unwrap()); // @depends-exact

    assert_ne!(
        Stopwatch::from_raw(dt, Some(anchor1)),
        Stopwatch::from_raw(dt, Some(anchor2)),
    );
}

#[test]
fn partial_eq_saturation_disqualifies_elapsed_as_viable_method() {
    let anchor0 = I::now();
    let anchor1 = Instant::checked_add(&anchor0, DELAY).unwrap();
    let anchor2 = Instant::checked_add(&anchor1, DELAY).unwrap();
    //     NOW
    // <----|--------->
    //      0   1   2
    let sw_1 = Stopwatch::new_started_at(anchor1);
    let sw_2 = Stopwatch::new_started_at(anchor2);

    // yes,
    assert_eq!(sw_1.elapsed_at(anchor0), sw_2.elapsed_at(anchor0));
    assert_eq!(sw_1.is_running(), sw_2.is_running());

    // but...
    assert_ne!(sw_1, sw_2);
}

#[test]
fn partial_eq_mixed_state() {
    let anchor1 = I::now();
    let anchor2 = Instant::checked_add(&anchor1, DELAY).unwrap();
    let sw_0 = Stopwatch::new();
    let sw_1 = Stopwatch::new_started_at(anchor1);
    let sw_2 = Stopwatch::new_started_at(anchor2);

    // yes,
    assert_eq!(sw_0.elapsed_at(anchor1), sw_1.elapsed_at(anchor1));
    assert_eq!(sw_0.elapsed_at(anchor1), sw_2.elapsed_at(anchor1));
    assert_eq!(sw_1.elapsed_at(anchor1), sw_2.elapsed_at(anchor1));

    // but...
    assert_ne!(sw_0, sw_1);
    assert_ne!(sw_0, sw_2);
    assert_ne!(sw_1, sw_2);
}

/* TODOO: find a canonicalized form for stopwatches where
 * `start.checked_sub(elapsed).is_none()`, so we can test equality as
 * expected */
#[ignore]
#[test]
fn unbounded_eq_future() {
    let anchor = I::now();
    let sw_1 = Stopwatch::from_raw(Duration::MAX, Some(anchor));
    let sw_2 = Stopwatch::from_raw(
        Duration::MAX - DELAY,
        Some(Instant::checked_sub(&anchor, DELAY).unwrap()),
    );
    let sw_3 = Stopwatch::from_raw(Duration::MAX - DELAY, Some(anchor));

    assert_eq!(sw_1, sw_2);
    assert_ne!(sw_1, sw_3);
    assert_ne!(sw_2, sw_3);
}

#[test]
fn unbounded_eq_status_quo() {
    let overflowing_1;
    let overflowing_2;
    {
        let start_1 = I::now();
        let start_2 = Instant::checked_sub(&start_1, DELAY).unwrap();
        overflowing_1 = Stopwatch::from_raw(Duration::MAX, Some(start_1));
        overflowing_2 = Stopwatch::from_raw(Duration::MAX, Some(start_2));
    }

    assert_eq!(overflowing_1, overflowing_2);
}

#[test]
fn partial_eq() {
    // TODO: this is testing... nothing? PartialEq::ne has a default impl which is ostensibly correct
    for [a, b, _] in mixed_stopwatches() {
        assert_eq!(a == b, !(a != b));
    }
}

#[test]
fn hash_and_eq() {
    for [sw_1, sw_2, sw_3] in mixed_stopwatches() {
        let mut hasher_1 = DefaultHasher::new();
        let mut hasher_2 = DefaultHasher::new();
        let mut hasher_3 = DefaultHasher::new();

        sw_1.hash(&mut hasher_1);
        sw_2.hash(&mut hasher_2);
        sw_3.hash(&mut hasher_3);

        dbg!(sw_1, sw_2, sw_3);

        // > When implementing both Hash and Eq, it is important that the following property holds:
        // > k1 == k2 -> hash(k1) == hash(k2)
        assert_eq!(sw_1 == sw_2, hasher_1.finish() == hasher_2.finish());
        assert_eq!(sw_1 == sw_3, hasher_1.finish() == hasher_3.finish());
        assert_eq!(sw_2 == sw_3, hasher_2.finish() == hasher_3.finish());
    }
}

#[test]
fn hash_running() {
    let now = I::now();
    let sw_1 = Stopwatch::new_started_at(now);
    let sw_2 = Stopwatch::new_started_at(now);
    let sw_3 = Stopwatch::from_raw(DELAY, Some(now));

    let mut hasher_1 = DefaultHasher::new();
    let mut hasher_2 = DefaultHasher::new();
    let mut hasher_3 = DefaultHasher::new();

    sw_1.hash(&mut hasher_1);
    sw_2.hash(&mut hasher_2);
    sw_3.hash(&mut hasher_3);

    // whatever is hashed shouldn't depend on the time of observation
    assert_eq!(hasher_1.finish(), hasher_2.finish());
    assert_ne!(hasher_1.finish(), hasher_3.finish());
}

fn mixed_stopwatches() -> [[Stopwatch; 3]; 11] {
    let crafted_1;
    let crafted_2;
    {
        let mut elapsed = Duration::from_secs(10);
        let mut start = I::now();
        crafted_1 = Stopwatch::from_raw(elapsed, Some(start));

        elapsed -= Duration::from_secs(1);
        start = Instant::checked_sub(&start, Duration::from_secs(1)).unwrap();
        crafted_2 = Stopwatch::from_raw(elapsed, Some(start));
    }
    assert_eq!(crafted_1, crafted_2);

    let started = Stopwatch::new_started();
    let started_elapsed_1 = Stopwatch::with_elapsed_started(Duration::from_secs(1));
    let started_elapsed_2 = Stopwatch::with_elapsed_started(Duration::from_secs(2));

    let overflowing_1;
    let overflowing_2;
    {
        let start_1 = I::now();
        let start_2 = Instant::checked_sub(&start_1, DELAY).unwrap();
        overflowing_1 = Stopwatch::from_raw(Duration::MAX, Some(start_1));
        overflowing_2 = Stopwatch::from_raw(Duration::MAX, Some(start_2));
    }

    [
        [Stopwatch::new(), Stopwatch::new(), Stopwatch::new()],
        [started, started, started],
        [started, Stopwatch::new(), Stopwatch::new()],
        [
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(1)),
        ],
        [started_elapsed_1, started_elapsed_1, started_elapsed_1],
        [started_elapsed_1, started_elapsed_2, started_elapsed_1],
        [overflowing_1, overflowing_2, started],
        [
            started_elapsed_1,
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(1)),
        ],
        [
            started_elapsed_2,
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(1)),
        ],
        [
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(2)),
            Stopwatch::with_elapsed(Duration::from_secs(3)),
        ],
        [crafted_1, crafted_2, Stopwatch::default()],
    ]
}
