use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound id: {0}")]
    NotFound(i32),
}