//! [`CartError`].

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CartError {
    #[error("cart integration is not configured")]
    NotConfigured,
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("cart request failed: {0}")]
    Request(String),
}
