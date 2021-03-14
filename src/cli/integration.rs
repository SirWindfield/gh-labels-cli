use crate::{cli::Cli, Result};
use clap::{AppSettings, Clap};
use eyre::{eyre, WrapErr};
use std::process::Command;
use terminal_link::Link;

/// Install `gh` CLI integrations.
#[derive(Clap, Clone, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct IntegrationArgs {
    #[clap(subcommand)]
    pub cmd: IntegrationSubCommand,
}

#[derive(Clap, Clone, Debug)]
pub enum IntegrationSubCommand {
    Install,
    Uninstall,
}

impl IntegrationArgs {
    pub fn run(self, _cli: Cli) -> Result<()> {
        match self.cmd {
            IntegrationSubCommand::Install => {
                // gh alias set labels -s 'gh-labels api $@'
                let exit_code = Command::new("gh")
                    .args(&["alias", "set", "labels", "-s", "'gh-labels api $@'"])
                    .status()
                    .wrap_err_with(|| {
                        format!(
                            "Make sure that the official GitHub CLI is installed: {}",
                            Link::new("Website", "https://cli.github.com")
                        )
                    })?;
                if !exit_code.success() {
                    return Err(eyre!(
                        "Calling `gh` CLI tool failed with a non-zero exit code"
                    ));
                }
            }
            IntegrationSubCommand::Uninstall => {
                // gh alias delete labels
                let exit_code = Command::new("gh")
                    .args(&["alias", "delete", "labels"])
                    .status()
                    .wrap_err_with(|| {
                        format!(
                            "Make sure that the official GitHub CLI is installed: {}",
                            Link::new("Website", "https://cli.github.com")
                        )
                    })?;
                if !exit_code.success() {
                    return Err(eyre!(
                        "Calling `gh` CLI tool failed with a non-zero exit code"
                    ));
                }
            }
        }

        Ok(())
    }
}
