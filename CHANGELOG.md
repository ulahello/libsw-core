# Changelog

## [Unreleased]

## [0.2.0] - 2024-12-17
### Added
- Added support for `tokio`
  - Added `tokio` feature flag
  - Added `TokioSw` type alias for `Stopwatch<tokio::time::Instant>`
  - Implemented `libsw_core::Instant` for `tokio::time::Instant`
- Added support for `coarsetime`
  - Added `coarsetime` feature flag
  - Added `CoarseSw` type alias for `Stopwatch<coarsetime::Instant>`
  - Implemented `libsw_core::Instant` for `coarsetime::Instant`
- Added support for `quanta`
  - Added `quanta` feature flag
  - Added `QuantaSw` type alias for `Stopwatch<quanta::Instant>`
  - Implemented `libsw_core::Instant` for `quanta::Instant`

### Changed
- Added more test coverage
- Updated documentation for `Stopwatch::start_at`
  - `libsw` will not repeat starts or stops, but `libsw-core` will. The precise
    semantics of this are now documented.

## [0.1.0] - 2024-09-12
### Added
- Added trait `Instant`
  - Added method `now`
  - Added method `checked_add`
  - Added method `checked_sub`
  - Added method `saturating_duration_since`
- Added generic struct `Stopwatch<I: Instant>`
  - Added public field `elapsed`
  - Added public field `start`
  - Added method `checked_add`
  - Added method `checked_elapsed`
  - Added method `checked_elapsed_at`
  - Added method `checked_stop`
  - Added method `checked_stop_at`
  - Added method `checked_sub`
  - Added method `checked_sub_at`
  - Added method `checked_toggle`
  - Added method `checked_toggle_at`
  - Added method `elapsed`
  - Added method `elapsed_at`
  - Added method `from_raw`
  - Added method `is_running`
  - Added method `is_stopped`
  - Added method `new`
  - Added method `new_started`
  - Added method `new_started_at`
  - Added method `replace`
  - Added method `replace_at`
  - Added method `reset`
  - Added method `reset_in_place`
  - Added method `reset_in_place_at`
  - Added method `saturating_add`
  - Added method `saturating_sub`
  - Added method `saturating_sub_at`
  - Added method `set`
  - Added method `set_in_place`
  - Added method `set_in_place_at`
  - Added method `start`
  - Added method `start_at`
  - Added method `stop`
  - Added method `stop_at`
  - Added method `toggle`
  - Added method `toggle_at`
  - Added method `with_elapsed`
  - Added method `with_elapsed_started`
  - Implemented trait `Add<Duration>`
  - Implemented trait `AddAssign<Duration>`
  - Implemented trait `Clone`
  - Implemented trait `Copy`
  - Implemented trait `Debug`
  - Implemented trait `Default`
  - Implemented trait `Eq`
  - Implemented trait `Hash`
  - Implemented trait `PartialEq`
  - Implemented trait `Sub<Duration>`
  - Implemented trait `SubAssign<Duration>`
- Added `std` feature flag
- Added `Sw` type alias for `Stopwatch<std::time::Instant>`
- Added `SystemSw` type alias for `Stopwatch<std::time::SystemTime>`
