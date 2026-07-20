//! [`TokenResponse`].

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub(crate) access_token: String,
}
