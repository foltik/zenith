#![feature(async_closure)]

pub use macros_zenith::main;

mod bridge;

pub mod log;

mod error;
pub use error::{Error, Result};

pub async fn init() -> Result<()> {
    log::init();
    bridge::init().await?;
    Ok(())
}

#[doc(hidden)]
pub use smol;
