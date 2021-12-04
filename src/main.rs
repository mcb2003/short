#[async_std::main]
async fn main() -> anyhow::Result<()> {
    use shoot::db::Link;

    let links = Link::all().await?;
    println!("{} links", links.len());
    Ok(())
}
