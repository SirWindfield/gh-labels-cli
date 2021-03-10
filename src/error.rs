use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Malformed repository")]
    InvalidRepoFormat,
    #[error("Label already exists: {0:?}")]
    LabelAlreadyExists(String),
    #[error("Failed to find GitHub API token")]
    NoTokenSpecified,
}
