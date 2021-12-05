mod api;

use anyhow::Context;
use tide::{Request, Response, StatusCode};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut app = tide::new();

    app.at("/api/v1").nest(api::install());
    app.at("/l/:slug").all(redirect);

    let bind_address = short::env("BIND_ADDRESS", "127.0.0.1:8080");
    app.listen(&bind_address)
        .await
        .with_context(|| format!("Failed to listen on {}", bind_address))?;
    Ok(())
}

async fn redirect(req: Request<()>) -> tide::Result {
    use short::db::Link;

    let slug = req.param("slug").unwrap().to_string();
    if let Some(link) = Link::by_slug(slug).await? {
        if !link.deleted() {
            Ok(Response::builder(StatusCode::Found)
                .header("Location", link.uri())
                .build())
        } else {
            Ok(Response::new(StatusCode::Gone))
        }
    } else {
        Ok(Response::new(StatusCode::NotFound))
    }
}
