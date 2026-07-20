//! Uniform JSON health reports for Sigma services.

mod check;
mod check_status;
mod health_report;
mod service_status;
pub use check::Check;
pub use check_status::CheckStatus;
pub use health_report::HealthReport;
pub use service_status::ServiceStatus;

use std::collections::BTreeMap;
use std::time::Instant;

use chrono::Utc;
use sqlx::PgPool;

/// Ping PostgreSQL and return a `database` check.
pub(crate) async fn check_database(pool: &PgPool) -> Check {
    let started = Instant::now();
    match super::ping(pool).await {
        Ok(()) => Check::healthy(Some(started.elapsed().as_millis() as u64)),
        Err(err) => Check::unhealthy(err.to_string()),
    }
}

/// Build a health report for a service with optional database connectivity.
pub async fn build_report(service: &str, database: Option<&PgPool>) -> HealthReport {
    let mut checks = BTreeMap::new();
    checks.insert("process".to_string(), Check::healthy(None));

    if let Some(pool) = database {
        checks.insert("database".to_string(), check_database(pool).await);
    }

    let status = overall_status(&checks);
    HealthReport {
        service: service.to_string(),
        status,
        checks,
        timestamp: Utc::now(),
    }
}

#[must_use]
pub(crate) fn overall_status(checks: &BTreeMap<String, Check>) -> ServiceStatus {
    let mut any_unhealthy = false;
    let mut any_unknown = false;
    for check in checks.values() {
        match check.status {
            CheckStatus::Healthy => {}
            CheckStatus::Unhealthy => any_unhealthy = true,
            CheckStatus::Unknown => any_unknown = true,
        }
    }
    if any_unhealthy {
        ServiceStatus::Unhealthy
    } else if any_unknown {
        ServiceStatus::Degraded
    } else {
        ServiceStatus::Healthy
    }
}

#[must_use]
pub fn http_status_code(report: &HealthReport) -> u16 {
    match report.status {
        ServiceStatus::Healthy => 200,
        ServiceStatus::Degraded => 200,
        ServiceStatus::Unhealthy => 503,
    }
}

#[cfg(feature = "warp")]
pub mod warp {
    use std::sync::Arc;

    use sqlx::PgPool;
    use warp::Filter;
    use warp::Rejection;
    use warp::Reply;
    use warp::http::StatusCode;

    use super::{build_report, http_status_code};

    /// `GET /health` returning a uniform JSON report.
    pub fn health_routes(
        service: &'static str,
        database: Option<Arc<PgPool>>,
    ) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        warp::path("health")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::any().map(move || (service, database.clone())))
            .then(
                |(service, database): (&'static str, Option<Arc<PgPool>>)| async move {
                    let pool_ref = database.as_deref();
                    let report = build_report(service, pool_ref).await;
                    let status =
                        StatusCode::from_u16(http_status_code(&report)).unwrap_or(StatusCode::OK);
                    warp::reply::with_status(warp::reply::json(&report), status)
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unhealthy_database_marks_service_unhealthy() {
        let mut checks = BTreeMap::new();
        checks.insert("process".to_string(), Check::healthy(None));
        checks.insert(
            "database".to_string(),
            Check::unhealthy("connection refused"),
        );
        assert_eq!(overall_status(&checks), ServiceStatus::Unhealthy);
    }
}
