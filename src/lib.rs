//! Shared PostgreSQL helpers for Sigma web services.

use std::env;

use anyhow::{Context, Result};
use serde::{Serialize, de::DeserializeOwned};
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

/// Apply embedded service document migrations.
pub async fn migrate(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("run sigma-pg migrations")?;
    Ok(())
}

/// Load a JSON document from `{schema}.document`.
pub async fn load_document<T: DeserializeOwned + Default>(
    pool: &PgPool,
    schema: &str,
) -> Result<T> {
    let table = qualified_table(schema, "document");
    let query = format!("SELECT data FROM {table} WHERE id = 1");
    let row: Option<serde_json::Value> = sqlx::query_scalar(&query)
        .fetch_optional(pool)
        .await
        .with_context(|| format!("load {schema} document"))?;

    match row {
        Some(value) => serde_json::from_value(value).context("deserialize document"),
        None => Ok(T::default()),
    }
}

/// Persist a JSON document to `{schema}.document`.
pub async fn save_document<T: Serialize + Sync>(pool: &PgPool, schema: &str, data: &T) -> Result<()> {
    let table = qualified_table(schema, "document");
    let query = format!(
        "INSERT INTO {table} (id, data, updated_at) VALUES (1, $1, now()) \
         ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data, updated_at = now()"
    );
    let value = serde_json::to_value(data).context("serialize document")?;
    sqlx::query(&query)
        .bind(value)
        .execute(pool)
        .await
        .with_context(|| format!("save {schema} document"))?;
    Ok(())
}

/// Fully qualified `{schema}.{table}` for dynamic SQL (schema may include quotes).
#[must_use]
pub fn qualified_table(schema: &str, table: &str) -> String {
    format!("{schema}.{table}")
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

    #[test]
    fn qualified_table_preserves_quoted_schema() {
        assert_eq!(
            qualified_table(r#""order""#, "document"),
            r#""order".document"#
        );
    }
}
