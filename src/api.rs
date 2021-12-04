use tide::{Body, Error, Request, Response};

use short::db::*;

pub fn install() -> tide::Server<()> {
    let mut app = tide::new();
        
app.at("/links").get(all_links).post(new_link);

        app
}

async fn all_links(_req: Request<()>) -> tide::Result {
    let links = Link::all().await?;
    let body = Body::from_json(&links)?;
    Ok(Response::builder(200).body(body).build())
}

async fn new_link(mut req: Request<()>) -> tide::Result {
    let new_link: NewLink = req.body_json().await.map_err(|_| Error::from_str(400, "Invalid JSON payload"))?;
    let link = new_link.save().await?;
    let body = Body::from_json(&link)?;
    Ok(Response::builder(201).body(body).build())
}
