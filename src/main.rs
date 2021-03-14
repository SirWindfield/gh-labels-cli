use crate::cli::{Cli, SubCommand};
use clap::Clap;

mod cli;
mod config;
mod error;
mod extension;
mod file;
mod util;

pub(crate) type Result<T> = eyre::Result<T>;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli: Cli = Cli::parse();
    match cli.cmd.clone() {
        SubCommand::Api(args) => args.run(cli).await?,
        SubCommand::Config(args) => args.run(cli)?,
    }

    Ok(())
}
