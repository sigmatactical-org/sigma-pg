//! Shared PostgreSQL connection helpers for Sigma web services.

use std::env;

use anyhow::{Context, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::debug;

/// Default migrator connection string when `DATABASE_URL` is unset.
pub const DEFAULT_DATABASE_URL: &str = "postgres://sigma:sigma@127.0.0.1:5432/sigma";

/// Default connection string for a service role (same host/db, role name as user).
#[must_use]
pub fn service_database_url(role: &str) -> String {
    format!("postgres://{role}:sigma@127.0.0.1:5432/sigma")
}

/// Read `DATABASE_URL` or fall back to [`DEFAULT_DATABASE_URL`].
#[must_use]
pub fn database_url_from_env() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_string())
}

/// Connect to PostgreSQL using `DATABASE_URL`.
pub async fn connect() -> Result<PgPool> {
    connect_url(&database_url_from_env()).await
}

/// Connect to PostgreSQL using an explicit URL.
///
/// Schema migrations run only when the connection user is `sigma` (the migrator
/// role), unless `SIGMA_PG_MIGRATE=1` is set.
pub async fn connect_url(database_url: &str) -> Result<PgPool> {
    debug!("Connecting to PostgreSQL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .with_context(|| format!("connect to PostgreSQL at {database_url}"))?;
    if should_auto_migrate(database_url) {
        migrate(&pool).await?;
    }
    Ok(pool)
}

/// Returns true when embedded migrations should run on connect.
#[must_use]
pub fn should_auto_migrate(database_url: &str) -> bool {
    if env::var("SIGMA_PG_MIGRATE").is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true")) {
        return true;
    }
    if env::var("SIGMA_PG_SKIP_MIGRATE").is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    {
        return false;
    }
    connection_role(database_url) == Some("sigma")
}

/// Apply embedded schema migrations.
pub async fn migrate(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("run sigma-pg migrations")?;
    Ok(())
}

/// Lightweight readiness probe for health endpoints.
pub async fn ping(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

fn connection_role(database_url: &str) -> Option<&str> {
    let rest = database_url.strip_prefix("postgres://")?;
    let user_pass_host = rest.split('@').next()?;
    user_pass_host.split(':').next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrator_runs_migrations() {
        assert!(should_auto_migrate(DEFAULT_DATABASE_URL));
    }

    #[test]
    fn service_roles_skip_migrations() {
        assert!(!should_auto_migrate(
            "postgres://catalog:sigma@127.0.0.1:5432/sigma"
        ));
    }
}
