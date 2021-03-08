use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Malformed repository")]
    InvalidRepoFormat,
    #[error("Failed to find GitHub API token")]
    NoTokenSpecified,
}
