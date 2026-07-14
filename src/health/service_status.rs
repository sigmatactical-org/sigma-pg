//! [`ServiceStatus`].

#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};

/// Overall service status derived from checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    #[default]
    Healthy,
    Degraded,
    Unhealthy,
}
