mod schema;
use schema::links;

use std::time::SystemTime;

use async_std::task;
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use diesel::prelude::*;
use once_cell::sync::Lazy;
use uuid::Uuid;

pub const DEFAULT_DB_URL: &'static str = concat!("postgres://localhost/", env!("CARGO_PKG_NAME"));

pub static DB_POOL: Lazy<Pool> = Lazy::new(|| {
    let db_url = crate::env("DATABASE_URL", DEFAULT_DB_URL);
    let manager = Manager::new(db_url, Runtime::AsyncStd1);
    Pool::builder(manager)
        .runtime(Runtime::AsyncStd1)
        .build()
        .expect("Failed to create database connection pool")
});

#[allow(dead_code)]
#[derive(Queryable, Identifiable, serde::Serialize)]
pub struct Link {
    id: Uuid,
    slug: Option<String>,
    uri: String,
    description: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}

impl Link {
    pub async fn all() -> anyhow::Result<Vec<Self>> {
        use schema::links::dsl::*;

        let conn = DB_POOL.get().await?;
        task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            Ok(links.load(&*conn)?)
        })
        .await
    }

    pub async fn by_id(uuid: Uuid) -> anyhow::Result<Option<Self>> {
        use schema::links::dsl::*;

        let conn = DB_POOL.get().await?;
        task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            Ok(links.find(uuid).load(&*conn).map(|v| v.into_iter().next())?)
        })
        .await
    }
}

#[derive(Default, Insertable, serde::Deserialize)]
#[table_name = "links"]
pub struct NewLink {
    pub slug: Option<String>,
    pub uri: String,
    pub description: String,
}

impl NewLink {
    pub async fn save(self) -> anyhow::Result<Link> {
        use schema::links::dsl::*;

        let conn = DB_POOL.get().await?;
        // Kinda annoying, this closure must be 'static, so we have to allocate and copy, even
        // though it's awaited straight away. Scoped tasks would be nice!
        task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            Ok(diesel::insert_into(links).values(self).get_result(&*conn)?)
        })
        .await
    }
}
