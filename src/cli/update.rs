use clap::{AppSettings, Clap};
use std::path::PathBuf;

/// Updates all labels from a label definition file.
#[derive(Clap, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct UpdateArgs {
    /// The label definitions file to use for updating.
    #[clap(long, short)]
    pub file: PathBuf,
}
