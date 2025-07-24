use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LocalResourceError {
    #[error("Failed to fetch local resource due to the following error: '{0}'")]
    FailedToFetch(String),
}
