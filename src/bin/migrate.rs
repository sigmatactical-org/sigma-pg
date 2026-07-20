#![forbid(unsafe_code)]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = sigma_pg::connect().await?;
    sigma_pg::migrate(&pool).await?;
    eprintln!("migrations applied");
    Ok(())
}
