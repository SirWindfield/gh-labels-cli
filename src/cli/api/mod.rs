use crate::{
    cli::Cli,
    util::{create_github_api_client, get_github_repo_and_owner},
    Result,
};
use clap::{AppSettings, Clap};

mod create;
mod update;

/// Interact with the GitHub API.
#[derive(Clap, Clone, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct ApiArgs {
    /// The git repository to apply the changes to.
    ///
    /// Can be either a git url or a string in the format `owner/repo`. If not
    /// set, the current directory is assumed to be a valid git repository and
    /// the remote url named `origin` will be taken.
    #[clap(long, short)]
    pub repo: String,

    /// The GitHub personal access token. Takes precedence over environment
    /// variables.
    #[clap(long, short)]
    pub token: Option<String>,

    #[clap(subcommand)]
    pub cmd: ApiSubCommand,
}

#[derive(Clap, Clone, Debug)]
pub enum ApiSubCommand {
    Create(create::CreateArgs),
    Update(update::UpdateArgs),
}

impl ApiArgs {
    pub async fn run(self, cli: Cli) -> Result<()> {
        let github = create_github_api_client(self.token.as_deref())?;
        let repo = get_github_repo_and_owner(&self.repo)?;
        let repo = github.repo(repo.0, repo.1);

        match self.cmd {
            ApiSubCommand::Create(args) => args.run(cli, repo).await?,
            ApiSubCommand::Update(args) => args.run(cli, repo).await?,
        }

        Ok(())
    }
}
