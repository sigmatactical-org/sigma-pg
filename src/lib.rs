//! Shared PostgreSQL helpers for Sigma web services.

use std::env;

use anyhow::{Context, Result};
use serde::{Serialize, de::DeserializeOwned};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::debug;

/// Default local connection string when `DATABASE_URL` is unset.
pub const DEFAULT_DATABASE_URL: &str = "postgres://sigma:sigma@127.0.0.1:5432/sigma";

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
pub async fn connect_url(database_url: &str) -> Result<PgPool> {
    debug!("Connecting to PostgreSQL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .with_context(|| format!("connect to PostgreSQL at {database_url}"))?;
    migrate(&pool).await?;
    Ok(pool)
}

/// Apply embedded service snapshot migrations.
pub async fn migrate(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("run sigma-pg migrations")?;
    Ok(())
}

/// Load a JSON document snapshot from `{schema}.snapshot`.
pub async fn load_snapshot<T: DeserializeOwned + Default>(
    pool: &PgPool,
    schema: &str,
) -> Result<T> {
    let query = format!("SELECT data FROM {schema}.snapshot WHERE id = 1");
    let row: Option<serde_json::Value> = sqlx::query_scalar(&query)
        .fetch_optional(pool)
        .await
        .with_context(|| format!("load {schema} snapshot"))?;

    match row {
        Some(value) => serde_json::from_value(value).context("deserialize snapshot"),
        None => Ok(T::default()),
    }
}

/// Persist a JSON document snapshot to `{schema}.snapshot`.
pub async fn save_snapshot<T: Serialize + Sync>(
    pool: &PgPool,
    schema: &str,
    data: &T,
) -> Result<()> {
    let value = serde_json::to_value(data).context("serialize snapshot")?;
    let query = format!(
        "INSERT INTO {schema}.snapshot (id, data, updated_at) VALUES (1, $1, now()) \
         ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data, updated_at = now()"
    );
    sqlx::query(&query)
        .bind(value)
        .execute(pool)
        .await
        .with_context(|| format!("save {schema} snapshot"))?;
    Ok(())
}

/// Lightweight readiness probe for health endpoints.
pub async fn ping(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
