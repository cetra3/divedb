use async_graphql::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use divedb_core::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct PasswordReset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Local>,
}
