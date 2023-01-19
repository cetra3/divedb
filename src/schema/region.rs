use async_graphql::*;
use divedb_core::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
pub struct Region {
    pub id: Uuid,
    pub name: String,
    pub lat_min: f64,
    pub lon_min: f64,
    pub lat_max: f64,
    pub lon_max: f64,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, InputObject)]
pub struct CreateRegion {
    pub id: Option<Uuid>,
    pub name: String,
    pub lat_min: f64,
    pub lon_min: f64,
    pub lat_max: f64,
    pub lon_max: f64,
}
