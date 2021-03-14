use crate::error::Error;
use eyre::{Context, Result};
use hubcaps::{Credentials, Github};
use std::{borrow::Cow, env};

const USER_AGENT: &str = "gh-labels-cli";

fn github_api_token(cli_token: Option<&str>) -> Option<Cow<'_, str>> {
    cli_token.map(Into::into).or_else(|| {
        env::var("GH_LABELS_TOKEN")
            .or_else(|_| env::var("GITHUB_TOKEN"))
            .ok()
            .map(Into::into)
    })
}

pub fn create_github_api_client(cli_token: Option<&str>) -> Result<Github> {
    let token = github_api_token(cli_token);
    match token {
        Some(token) => Github::new(USER_AGENT, Credentials::Token(token.to_string())).wrap_err_with(|| "Failed to create GitHub API client"),
        None => Err(Error::NoTokenSpecified).wrap_err_with(|| "Make sure to either set the API token via the environment variables `GH_LABELS_TOKEN` or `GITHUB_TOKEN` or pass the token to the CLI via the `-t,--token` flag.")
    }
}

pub type GitHubRepo<'a> = (&'a str, &'a str);

/// Parses the repository CLI argument and constructs a GitHubRepo instance.
///
/// # Returns
///
/// `Ok(GitHubRepo)` if the argument had the right format,
/// `Err(Error::InvalidRepoFormat)` otherwise.
fn github_repo_from_cli_arg(arg: &str) -> std::result::Result<GitHubRepo<'_>, Error> {
    let number_of_slashes = arg.matches('/').count();

    match number_of_slashes {
        1 => {
            // Safety: match arm.
            let slash_index = arg.find('/').unwrap();
            Ok((&arg[..slash_index], &arg[slash_index + 1..]))
        }
        _ => Err(Error::InvalidRepoFormat),
    }
}

pub fn get_github_repo_and_owner(repo_arg: &str) -> Result<GitHubRepo<'_>> {
    github_repo_from_cli_arg(repo_arg).wrap_err_with(|| {
        "The repository field has to be provided as `owner/repo` or as a Git URL!"
    })
}
