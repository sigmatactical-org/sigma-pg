#![forbid(unsafe_code)]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Plain pool from `database_url_from_env()` — no auto-migrate on connect,
    // so a reset can still recover a database with broken migration history.
    let url = sigma_pg::database_url_from_env();
    let pool = sqlx::postgres::PgPoolOptions::new().connect(&url).await?;
    sigma_pg::reset_and_migrate(&pool).await?;
    eprintln!("database reset and migrations applied");
    Ok(())
}
