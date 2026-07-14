//! [`IdentityStatus`].

#[allow(unused_imports)]
use super::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct IdentityStatus {
    pub authenticated: bool,
    pub username: Option<String>,
    pub email: Option<String>,
    pub user_id: Option<String>,
}
