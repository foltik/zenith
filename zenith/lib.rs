#![feature(async_closure)]

#[cfg(feature = "bin")]
#[doc(hidden)]
pub mod bin;

#[cfg(feature = "bin")]
pub use bin::main;

#[cfg(feature = "bin")]
pub use clap;
