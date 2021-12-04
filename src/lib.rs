#[macro_use]
extern crate diesel;

mod schema;

use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use once_cell::sync::Lazy;

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
