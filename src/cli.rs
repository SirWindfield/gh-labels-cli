use clap::{AppSettings, Clap};
use std::path::PathBuf;

#[derive(Clap, Debug)]
pub enum SubCommand {
    Import(ImportArgs),
}

#[derive(Clap, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct ImportArgs {
    #[clap(long, short)]
    pub file: PathBuf,
    pub repo: String,
}

#[derive(Clap, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: SubCommand,

    #[clap(long, short)]
    pub token: Option<String>,
}
