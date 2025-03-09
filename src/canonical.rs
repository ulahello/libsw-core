use core::hash::{Hash, Hasher};
use core::mem;
use core::time::Duration;

use crate::{Instant, Stopwatch};

fn instant_eq<I: Instant>(lhs: I, rhs: I) -> bool {
    lhs.saturating_duration_since(rhs) == rhs.saturating_duration_since(lhs)
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Canonical<I: Instant> {
    Stopped(Duration),
    Bounded(I),
    Unbounded(()),
}

impl<I: Instant> Canonical<I> {
    pub fn new(sw: Stopwatch<I>) -> Self {
        match sw.start {
            None => Self::Stopped(sw.elapsed),

            Some(start) => match start.checked_sub(sw.elapsed) {
                Some(sum) => {
                    // # Case 1: t - d ∈ T
                    // all of the duration can be "moved" to the instant,
                    // leaving an implicit zero.
                    Self::Bounded(sum)
                }

                None => {
                    // # Case 2: t - d ∉ T
                    // we consider all unbounded stopwatches to be equivalent.
                    // it's tricky to do otherwise because of how opaque and
                    // generic Instants are.
                    Self::Unbounded(())
                }
            },
        }
    }
}

impl<I: Instant> PartialEq for Canonical<I> {
    fn eq(&self, rhs: &Self) -> bool {
        if mem::discriminant::<Self>(self) != mem::discriminant::<Self>(rhs) {
            return false;
        }

        match (*self, *rhs) {
            (Self::Stopped(lhs), Self::Stopped(rhs)) => lhs == rhs,
            (Self::Bounded(lhs), Self::Bounded(rhs)) => instant_eq(lhs, rhs),
            (Self::Unbounded(()), Self::Unbounded(())) => true,
            _ => unreachable!(),
        }
    }
}

impl<I: Instant + Hash> Hash for Canonical<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let tag = mem::discriminant::<Canonical<I>>(self);

        tag.hash(state);

        match self {
            Self::Stopped(d) => d.hash(state),
            Self::Bounded(t) => t.hash(state),
            Self::Unbounded(()) => {}
        }
    }
}
