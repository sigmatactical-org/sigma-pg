#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| sigma_pg::DEFAULT_DATABASE_URL.to_string());
    let pool = sqlx::postgres::PgPoolOptions::new().connect(&url).await?;
    sigma_pg::migrate(&pool).await?;
    eprintln!("migrations applied");
    Ok(())
}
