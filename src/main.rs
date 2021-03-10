use crate::{
    cli::{Cli, SubCommand},
    error::Error,
    file::{read_file, Label},
    util::{create_github_api_client, get_github_repo_and_owner, LabelAlreadyExistsError},
};
use clap::Clap;
use eyre::{Result, WrapErr};
use hubcaps::Error as GithubError;
use terminal_emoji::Emoji;

mod cli;
mod error;
mod file;
mod util;

const ERROR_EMOJI: Emoji = Emoji::new("✖", "×");
const INFO_EMOJI: Emoji = Emoji::new("ℹ", "i");
const SUCCESS_EMOJI: Emoji = Emoji::new("✔", "√");
const WARNING_EMOJI: Emoji = Emoji::new("⚠", "‼");

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli: Cli = Cli::parse();

    let github = create_github_api_client(cli.token.as_deref())?;
    let repo = get_github_repo_and_owner(&cli.repo)?;
    let repo = github.repo(repo.0, repo.1);

    match cli.cmd {
        SubCommand::Create(args) => {
            let label = Label::from(args.color, args.description, args.name);
            let label_name = label.name.clone();

            let res = repo.labels().create(&label.into()).await;
            match res {
                Err(e) => match e {
                    GithubError::Fault { error, .. } => {
                        if error.label_already_exists() {
                            return Err(Error::LabelAlreadyExists(label_name)).wrap_err_with(
                                || "GitHub doesn't support multiple labels with the same name",
                            );
                        }
                    }
                    _ => {
                        eprintln!("handle other errors!")
                    }
                },
                _ => {
                    println!("{} Created label {:?}", SUCCESS_EMOJI, label_name);
                }
            }
        }
        SubCommand::Update(args) => {
            let labels = read_file(&args.file)?;
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
