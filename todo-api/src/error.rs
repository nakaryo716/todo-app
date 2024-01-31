use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound")]
    NotFound,
    #[error("Unexpected error")]
    Unexpected,
    #[error("Database error: {0}")]
    DatabaseError(String),
}
