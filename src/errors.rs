use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LocalResourceError {
    #[error("Failed to fetch local resource at '{0}'")]
    FailedToFetch(String),
}
