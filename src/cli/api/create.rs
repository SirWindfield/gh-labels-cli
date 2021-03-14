use crate::{
    cli::Cli,
    error::Error,
    extension::{LabelAlreadyExistsExt, RepoNotFoundExt},
    file::JsonLabel,
    Result,
};
use clap::{AppSettings, Clap};
use eyre::WrapErr;
use hubcaps::repositories::Repository;
use terminal_log_symbols::colored::SUCCESS_SYMBOL;

/// Create a new label.
#[derive(Clap, Clone, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct CreateArgs {
    /// The color of the label in hex notation (without the hash).
    #[clap(long, short)]
    pub color: String,

    /// The description of the label.
    #[clap(long, short)]
    pub description: Option<String>,

    /// The name of the label.
    #[clap(long, short)]
    pub name: String,

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
}

impl CreateArgs {
    pub async fn run(self, _cli: Cli, repo: Repository) -> Result<()> {
        let label = JsonLabel::from(
            self.color,
            self.description.unwrap_or_else(|| "".into()),
            self.name,
        );
        let label_name = label.name.clone();

        let res = repo.labels().create(&label.into()).await;
        match res {
            Err(e) => {
                if e.is_label_already_exists_error() {
                    return Err(Error::LabelAlreadyExists(label_name)).wrap_err_with(|| {
                        "GitHub doesn't support multiple labels with the same name"
                    });
                } else if e.is_repo_not_found_error() {
                    return Err(Error::RepoNotFound(self.repo.clone())).wrap_err_with(|| {
                        "Make sure that the repository does exist before using the CLI"
                    });
                }

                return Err(Error::ApiError(e)).wrap_err_with(|| {
                    "Something went wrong during label creation. Please try again."
                });
            }
            _ => {
                println!("{} Created label {:?}", SUCCESS_SYMBOL, label_name);
            }
        }

        Ok(())
    }
}
