// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
mod std_instant;

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
mod std_systemtime;

#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
mod tokio;

#[cfg(feature = "coarsetime")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "coarsetime")))]
mod coarsetime;

#[cfg(feature = "quanta")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "quanta")))]
mod quanta;

#[allow(deprecated)]
#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
mod time;
