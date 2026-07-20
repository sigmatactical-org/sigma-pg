//! [`HealthReport`].

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::{Check, ServiceStatus};

/// Standard `/health` response body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthReport {
    pub service: String,
    pub status: ServiceStatus,
    pub checks: BTreeMap<String, Check>,
    pub timestamp: DateTime<Utc>,
}
