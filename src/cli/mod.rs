use clap::{AppSettings, Clap};

mod config;
mod create;
mod update;

#[derive(Clap, Clone, Debug)]
pub enum SubCommand {
    Config(config::ConfigArgs),
    Create(create::CreateArgs),
    Update(update::UpdateArgs),
}

#[derive(Clap, Clone, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: SubCommand,

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
