//! [`CatalogError`].

#[allow(unused_imports)]
use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("catalog integration is not configured")]
    NotConfigured,
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("catalog request failed: {0}")]
    Request(String),
}
