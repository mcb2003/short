#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let conn = shoot::DB_POOL.get().await?;
    Ok(())
}
