// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

//! `libsw_core` is a comprehensive stopwatch implementation.
//!
//! It offers [checked stopping](Stopwatch::checked_stop) and
//! [arithmetic](Stopwatch::checked_add), [precise
//! control](Stopwatch::start_at) over when operations occur, and supports
//! [arbitrary timekeeping types](Instant).
//!
//! If you want to do benchmarking, please use something like
//! [Criterion](https://docs.rs/criterion).
//!
//! # Introduction
//!
//! `libsw_core` provides the [`Stopwatch`] type.
//!
//! This implementation is agnostic to the timekeeping type used, by
//! virtue of being generic. Any type `I` that implements the [`Instant`]
//! trait (as in `Stopwatch<I>`) can be used for timekeeping.
//!
//! `Instant` is implemented for timekeeping types from the standard
//! library out of the box. These implementations are exposed as type
//! aliases.
//!
//! # Features
//!
//! | Name         | Implies | Description                                                                                                                               |
//! |--------------|---------|-------------------------------------------------------------------------------------------------------------------------------------------|
//! | `default`    |         | Enabled by default.                                                                                                                       |
//! | `std`        |         | Depends on the standard library. Implements [`Instant`] for `std::time::{Instant, SystemTime}`. Exposes `Sw` and `SystemSw` type aliases. |
//! | `tokio`      | `std`   | Implements [`Instant`] for `tokio::time::Instant`. Exposes `TokioSw` type alias.                                                          |
//! | `coarsetime` | `std`   | Implements [`Instant`] for `coarsetime::Instant`. Exposes `CoarseSw` type alias.                                                          |
//! | `quanta`     | `std`   | Implements [`Instant`] for `quanta::Instant`. Exposes `QuantaSw` type alias.                                                              |
//! | `time`       | `std`   | Deprecated. Implements [`Instant`] for `time::Instant`. Exposes `TimeSw` type alias.                                                      |
//!
//! ## `no_std` support
//!
//! `#![no_std]` is set by default.
//!
//! ## Compiler support
//!
//! Standalone, the minimum supported version of Rust is `1.61.0`.
//! Adding dependencies may bump this.
//!
//! ## Safety
//!
//! `libsw_core` contains no unsafe code (`#![forbid(unsafe_code)]`).

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic, clippy::cargo)]

extern crate core;

mod instant;
mod instant_impls;
mod stopwatch;

pub use crate::instant::Instant;
pub use crate::stopwatch::Stopwatch;

/// Alias to [`Stopwatch`] using the standard library's
/// [`Instant`](std::time::Instant) type.
#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
pub type Sw = Stopwatch<::std::time::Instant>;

/// Alias to [`Stopwatch`] using the standard library's
/// [`SystemTime`](std::time::SystemTime) type.
#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
pub type SystemSw = Stopwatch<::std::time::SystemTime>;

/// Alias to [`Stopwatch`] using Tokio's [`Instant`](tokio::time::Instant) type.
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
pub type TokioSw = Stopwatch<::tokio::time::Instant>;

/// Alias to [`Stopwatch`] using the `coarsetime` crate's
/// [`Instant`](coarsetime::Instant) type.
#[cfg(feature = "coarsetime")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "coarsetime")))]
pub type CoarseSw = Stopwatch<::coarsetime::Instant>;

/// Alias to [`Stopwatch`] using the `quanta` crate's
/// [`Instant`](quanta::Instant) type.
#[cfg(feature = "quanta")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "quanta")))]
pub type QuantaSw = Stopwatch<::quanta::Instant>;

/// Alias to [`Stopwatch`] using the `time` crate's [`Instant`](time::Instant)
/// type.
#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
#[deprecated(
    note = "the `time` crate has deprecated `time::Instant` in favor of the `time::ext::InstantExt` trait used with `std::time::Instant`"
)]
pub type TimeSw = Stopwatch<::time::Instant>;

#[cfg(test)]
mod tests;
