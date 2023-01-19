use anyhow::Error;
pub use divedb_macro::*;
use tokio_postgres::Row;

/// Really simple ORM for `tokio_postgres`
pub trait FromRow {
    /// hydrate a struct a database `Row`
    fn from_row(row: Row) -> Result<Self, Error>
    where
        Self: std::marker::Sized;

    /// hydrate a `Vec` of structs from database `Row` list
    fn from_rows(rows: Vec<Row>) -> Result<Vec<Self>, Error>
    where
        Self: std::marker::Sized,
    {
        rows.into_iter().map(Self::from_row).collect()
    }

    /// Simple reflection to see what fields a struct has
    fn fields() -> &'static [&'static str];
}
