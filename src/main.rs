use crate::{
    cli::{Cli, SubCommand},
    error::Error,
    file::read_file,
    util::{github_api_token, github_repo_from_cli_arg},
};
use clap::Clap;
use eyre::{Result, WrapErr};
use hubcaps::{Credentials, Github};

mod cli;
mod error;
mod file;
mod util;

const USER_AGENT: &str = "gh-labels-cli";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli: Cli = Cli::parse();
    match cli.cmd {
        SubCommand::Import(args) => {
            let token = github_api_token(cli.token.as_deref());
            if token.is_none() {
                return Err(Error::NoTokenSpecified).wrap_err_with(|| "Make sure to either set the API token via the environment variables `GH_LABELS_TOKEN` or `GITHUB_TOKEN` or pass the token to the CLI via the `-t,--token` flag.");
            }

            let github = Github::new(USER_AGENT, Credentials::Token(token.unwrap().to_string()))
                .wrap_err_with(|| "Failed to create GitHub API client")?;
            let labels = read_file(&args.file)?;

            // Fetch the list of labels.
            let repo = github_repo_from_cli_arg(&args.repo)
                .wrap_err_with(|| "The repository field has to be provided as `owner/repo`!")?;
            let repo = github.repo(repo.0, repo.1);
            let repo_labels = repo.labels();

            // Create each label that has been read from the label definition file.
            // TODO: filter for already existing labels and ignore those.
            // TODO: collect all errors and fail once at the end.
            // TODO: show nice emoji for each label created :)
            // TODO: add spinner to show wip.
            // TODO: throw error if repository doesn't exist.
            for label in labels {
                let label_name = label.name.clone();
                repo_labels
                    .create(&label.into())
                    .await
                    .wrap_err_with(|| format!("Failed to create label: {:?}", &label_name))?;
            }
        }
    }

    Ok(())
}
