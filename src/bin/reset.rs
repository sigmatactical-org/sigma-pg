#![forbid(unsafe_code)]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| sigma_pg::DEFAULT_DATABASE_URL.to_string());
    let pool = sqlx::postgres::PgPoolOptions::new().connect(&url).await?;
    sigma_pg::reset_and_migrate(&pool).await?;
    eprintln!("database reset and migrations applied");
    Ok(())
}
