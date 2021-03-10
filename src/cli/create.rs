use clap::{AppSettings, Clap};

/// Create a new label.
#[derive(Clap, Debug)]
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
}
