use substrate::{
    error::Result,
    log::{self, Level},
};
use zenith::clap;

#[derive(clap::Parser)]
struct Args {
    pub foo: usize,
}

#[zenith::main(args = Args)]
async fn main(args: Args) -> Result<()> {
    let (conn, screen) = x11rb::connect(None).expect("failed to connect to X server");

    log::event!(Level::INFO, "Hello, world: {}", args.foo);

    Ok(())
}
