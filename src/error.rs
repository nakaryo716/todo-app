use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unecpected error")]
    Unexpected,
    #[error("NotFound")]
    NotFound,
}
