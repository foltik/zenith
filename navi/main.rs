use stud::{
    args::clap,
    error::Result,
    log::{self, Level},
};

#[derive(Debug, clap::Parser)]
struct Args {}

#[zenith::main(args = Args)]
async fn main(args: Args) -> Result<()> {
    let (_conn, screen) = x11rb::connect(None).expect("failed to connect to X server");

    log::event!(Level::INFO, "Hello, X server!");
    log::event!(Level::INFO, "args: {args:?}");
    log::event!(Level::INFO, "screen: {screen:?}");

    Ok(())
}
