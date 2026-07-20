//! [`StorefrontError`].

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorefrontError {
    #[error("store integration is not configured")]
    NotConfigured,
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("store request failed: {0}")]
    Request(String),
}
