mod api;

use anyhow::Context;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut app = tide::new();

    app.at("/api/v1").nest(api::install());

    let bind_address = short::env("BIND_ADDRESS", "127.0.0.1:8080");
    app.listen(&bind_address).await.with_context(|| format!("Failed to listen on {}", bind_address))?;
    Ok(())
}
