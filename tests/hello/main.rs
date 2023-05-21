use zenith::{Result, log};

#[zenith::main]
async fn main() -> Result<()> {
    log::info!("Hello, world!");
    Ok(())
}
