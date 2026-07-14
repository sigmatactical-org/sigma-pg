//! [`CheckStatus`].

#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};

/// Result of a single dependency check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Healthy,
    Unhealthy,
    Unknown,
}
