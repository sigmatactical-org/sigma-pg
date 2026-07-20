//! [`ServiceStatus`].

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
impl ServiceStatus {
    /// Lowercase name matching the serde wire representation
    /// (`healthy` / `degraded` / `unhealthy`).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceStatus;

    #[test]
    fn as_str_matches_serde_representation() {
        for status in [
            ServiceStatus::Healthy,
            ServiceStatus::Degraded,
            ServiceStatus::Unhealthy,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            assert_eq!(json, format!("\"{}\"", status.as_str()));
        }
    }
}
