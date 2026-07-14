//! [`Check`].

#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};

/// One named check (for example `database`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Check {
    pub status: CheckStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Check {
    #[must_use]
    pub fn healthy(latency_ms: Option<u64>) -> Self {
        Self {
            status: CheckStatus::Healthy,
            latency_ms,
            message: None,
        }
    }

    #[must_use]
    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Unhealthy,
            latency_ms: None,
            message: Some(message.into()),
        }
    }
}
