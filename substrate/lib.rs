#![feature(exclusive_range_pattern)]

#[cfg(feature = "error")]
pub mod error;

#[cfg(feature = "log")]
pub mod log;

#[cfg(feature = "macros")]
pub mod macros;
