pub mod db;

#[macro_use]
extern crate diesel;

use chrono::{DateTime, NaiveDateTime, Utc};

pub fn env<T: ToString>(var: &str, default: T) -> String {
    std::env::var(var).unwrap_or_else(|_| default.to_string())
}

/// Serialises a naive timestamp, assuming it's UTC.
fn serialize_datetime_utc<S>(datetime: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;
    let dt = DateTime::<Utc>::from_utc(*datetime, Utc);
    dt.serialize(s)
}
