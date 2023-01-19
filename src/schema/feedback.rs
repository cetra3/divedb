use async_graphql::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use divedb_core::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject, FromRow)]
pub struct Feedback {
    pub id: Uuid,
    pub date: DateTime<Local>,
    pub user_id: Uuid,
    pub feedback: String,
}
