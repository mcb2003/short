use tide::{Body, Error, Request, Response, StatusCode};
use uuid::Uuid;

use short::db::*;

pub fn install() -> tide::Server<()> {
    let mut app = tide::new();
        
app.at("/links").get(all_links).post(new_link);
app.at("/links/:id").get(view_link).put(update_link).delete(delete_link);

        app
}

async fn all_links(_req: Request<()>) -> tide::Result {
    let links = Link::all().await?;
    let body = Body::from_json(&links)?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

async fn new_link(mut req: Request<()>) -> tide::Result {
    let new_link: NewLink = req.body_json().await.map_err(|_| Error::from_str(StatusCode::BadRequest, "Invalid JSON payload"))?;
    let link = new_link.save().await?;
    let body = Body::from_json(&link)?;
    Ok(Response::builder(StatusCode::Created).body(body).build())
}

async fn view_link(req: Request<()>) -> tide::Result {
    let id: Uuid = req.param("id").unwrap().parse().map_err(|e| Error::new(StatusCode::BadRequest, e))?;
    Ok(if let Some(link) = Link::by_id(id).await? {
        if !link.deleted() {
        let body = Body::from_json(&link)?;
        Response::builder(StatusCode::Ok).header("Last-Modified", link.updated_at().to_rfc2822()).body(body).build()
        } else {
            Response::new(StatusCode::Gone)
        }
    } else {
        Response::new(StatusCode::NotFound)
    })
}

async fn update_link(mut req: Request<()>) -> tide::Result {
    let id: Uuid = req.param("id").unwrap().parse().map_err(|e| Error::new(StatusCode::BadRequest, e))?;
    let last_modified = if let Some(date) = req.header("If-Unmodified-Since") {
        let date = date.last();
        chrono::DateTime::parse_from_rfc2822(date.as_str()).map_err(|_| Error::from_str(StatusCode::BadRequest, "Invalid date in If-Unmodified-Since header"))?.naive_utc()
    } else {
        return Err(Error::from_str(StatusCode::PreconditionRequired, "Must provide an If-Unmodified-Since header"));
    };
    let update: LinkUpdate = if !Link::is_id_deleted(id).await? {
     req.body_json().await.map_err(|_| Error::from_str(StatusCode::BadRequest, "Invalid JSON payload"))?
    } else {
        // Link was deleted
        return Ok(Response::new(StatusCode::Gone));
    };
    Ok(if let Some(new) = update.update_if(id, last_modified).await? {
        // Link hasn't been modified since last_modified
    let body = Body::from_json(&new)?;
    Response::builder(StatusCode::Ok).body(body).header("Last-Modified", new.updated_at().to_rfc2822()).build()
    } else {
        // Link was modified, client will have to refetch and try again
        Response::new(StatusCode::PreconditionFailed)
    })
}

async fn delete_link(req: Request<()>) -> tide::Result {
    let id: Uuid = req.param("id").unwrap().parse().map_err(|e| Error::new(StatusCode::BadRequest, e))?;
    if !Link::id_exists(id).await? {
        return Ok(Response::new(StatusCode::NotFound));
    }

    let last_modified = if let Some(date) = req.header("If-Unmodified-Since") {
        let date = date.last();
        chrono::DateTime::parse_from_rfc2822(date.as_str()).map_err(|_| Error::from_str(StatusCode::BadRequest, "Invalid date in If-Unmodified-Since header"))?.naive_utc()
    } else {
        return Err(Error::from_str(StatusCode::PreconditionRequired, "Must provide an If-Unmodified-Since header"));
    };

    Ok(if Link::delete_if(id, last_modified).await? {
        // Link hasn't been modified since last_modified
    Response::new(StatusCode::NoContent)
    } else {
        // Link was modified, client will have to refetch and try again
        Response::new(StatusCode::PreconditionFailed)
    })
    }
