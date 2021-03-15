use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error while communicating with GitHub API")]
    ApiError(#[source] hubcaps::Error),
    #[error("Malformed repository")]
    InvalidRepoFormat,
    #[error("Label already exists: {0:?}")]
    LabelAlreadyExists(String),
    #[error("Failed to find GitHub API token")]
    NoTokenSpecified,
    #[error("Repository not found: {0:?}")]
    RepoNotFound(String),
    #[error("No push access to repository: {0:?}")]
    Unauthorized(String),
}
