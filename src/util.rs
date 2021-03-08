use crate::error::Error;
use std::{borrow::Cow, env};

pub fn github_api_token(cli_token: Option<&str>) -> Option<Cow<'_, str>> {
    cli_token.map(Into::into).or_else(|| {
        env::var("GH_LABELS_TOKEN")
            .or_else(|_| env::var("GITHUB_TOKEN"))
            .ok()
            .map(Into::into)
    })
}

pub type GitHubRepo<'a> = (&'a str, &'a str);

pub fn github_repo_from_cli_arg(arg: &str) -> Result<GitHubRepo<'_>, Error> {
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
