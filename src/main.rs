use crate::{
    cli::{Cli, SubCommand},
    util::{create_github_api_client, get_github_repo_and_owner},
};
use clap::Clap;

mod cli;
mod error;
mod extension;
mod file;
mod util;

pub(crate) type Result<T> = eyre::Result<T>;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli: Cli = Cli::parse();

    let github = create_github_api_client(cli.token.as_deref())?;
    let repo = get_github_repo_and_owner(&cli.repo)?;
    let repo = github.repo(repo.0, repo.1);

    match cli.cmd.clone() {
        SubCommand::Create(args) => {
            args.run(cli, repo).await?;
        }
        SubCommand::Update(args) => {
            args.run(cli, repo).await?;
        }
    }

    Ok(())
}
