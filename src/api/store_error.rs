//! [`StoreError`].

use thiserror::Error;

/// Store-layer error shared by Sigma services; the [`StoreError::NotFound`]
/// payload names the missing entity (e.g. `"address"`, `"payment method"`).
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("{0} not found")]
    NotFound(&'static str),
    #[error("{0}")]
    InvalidInput(String),
    #[error("database error: {0}")]
    Database(#[from] anyhow::Error),
}
impl From<sqlx::Error> for StoreError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err.into())
    }
}
