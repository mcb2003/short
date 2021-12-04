pub mod db;

#[macro_use]
extern crate diesel;

pub fn env<T: ToString>(var: &str, default: T) -> String {
    std::env::var(var).unwrap_or_else(|_| default.to_string())
}
