//! [`SessionError`].

#[allow(unused_imports)]
use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("identity BFF is not configured")]
    NotConfigured,
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("identity status request failed: {0}")]
    Request(String),
}
