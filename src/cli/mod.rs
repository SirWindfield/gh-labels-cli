use clap::{AppSettings, Clap};

mod api;
mod config;
mod integration;

#[derive(Clap, Clone, Debug)]
pub enum SubCommand {
    Api(api::ApiArgs),
    Config(config::ConfigArgs),
    Integration(integration::IntegrationArgs),
}

#[derive(Clap, Clone, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}
