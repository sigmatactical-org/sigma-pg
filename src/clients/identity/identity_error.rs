//! [`IdentityError`].

#[allow(unused_imports)]
use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("identity integration is not configured")]
    NotConfigured,
    #[error("invalid issuer URL: {0}")]
    InvalidIssuer(String),
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Keycloak token request failed: {0}")]
    Token(String),
    #[error("Keycloak user listing failed: {0}")]
    Users(String),
}
