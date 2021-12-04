#[macro_use]
extern crate diesel;

mod schema;
use schema::links;

use std::time::SystemTime;

use async_std::task;
use diesel::prelude::*;
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use once_cell::sync::Lazy;
use uuid::Uuid;

pub fn env<T: ToString>(var: &str, default: T) -> String {
    std::env::var(var).unwrap_or_else(|_| default.to_string())
}

pub const DEFAULT_DB_URL: &'static str = concat!("postgres://localhost/", env!("CARGO_PKG_NAME"));

pub static DB_POOL: Lazy<Pool> = Lazy::new(|| {
    let db_url = env("DATABASE_URL", DEFAULT_DB_URL);
    let manager = Manager::new(db_url, Runtime::AsyncStd1);
    Pool::builder(manager)
        .runtime(Runtime::AsyncStd1)
        .build()
        .expect("Failed to create database connection pool")
});

#[allow(dead_code)]
#[derive(Queryable, Identifiable)]
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
        }).await
    }
}