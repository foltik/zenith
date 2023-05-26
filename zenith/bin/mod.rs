use clap::{ArgMatches, Args, Command, FromArgMatches, Parser};

pub use stud::error::Result;
pub use stud::rt::run;
pub use zenith_macros::main;

#[derive(Debug, Args)]
pub struct ZenithArgs {
    /// log filter. overrides verbose/quiet levels
    #[arg(short = 'V')]
    pub filter: Option<String>,

    /// log verbosity level, repeatable
    #[arg(short, action = clap::ArgAction::Count, default_value_t = 0)]
    pub verbose: u8,

    /// log quiet level, repeatable
    #[arg(short, action = clap::ArgAction::Count, default_value_t = 0)]
    pub quiet: u8,
}

fn parse<A: FromArgMatches>(cmd: &mut Command, matches: &mut ArgMatches) -> A {
    match A::from_arg_matches_mut(matches) {
        Ok(a) => a,
        Err(e) => e.format(cmd).exit(),
    }
}

async fn _init(module: &'static str, z_args: ZenithArgs) -> Result<()> {
    stud::log::init(module, z_args.verbose as i8 - z_args.quiet as i8, None)?;
    Ok(())
}

pub async fn init(module: &'static str) -> Result<()> {
    let cmd = Command::new(module);
    let mut cmd = ZenithArgs::augment_args(cmd);
    let mut matches = cmd.get_matches_mut();

    let z_args = parse::<ZenithArgs>(&mut cmd, &mut matches);

    _init(module, z_args).await
}

pub async fn init_args<A: Parser>(module: &'static str) -> Result<A> {
    let cmd = A::command();
    let mut cmd = ZenithArgs::augment_args(cmd);
    let mut matches = cmd.get_matches_mut();

    let z_args = parse::<ZenithArgs>(&mut cmd, &mut matches);
    let args = parse::<A>(&mut cmd, &mut matches);

    _init(module, z_args).await?;

    Ok(args)
}
