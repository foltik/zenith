use crate::error::{Context, Result};
use crate::log;

use futures::{Future, FutureExt};
use tokio::runtime::{Builder, Runtime};

pub fn run(f: impl Future<Output = Result<()>>) -> Result<()> {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("failed to create tokio runtime")?;

    Runtime::new().unwrap().block_on(async move {
        futures::select! {
            result = f.fuse() => result,
            _ = tokio::signal::ctrl_c().fuse() => {
                log::debug!("exiting: ctrl-c received");
                Ok(())
            }
        }
    })
}
