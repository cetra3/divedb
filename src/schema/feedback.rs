use async_graphql::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use divedb_core::FromRow;

use crate::graphql::SchemaContext;

use super::UserInfo;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Feedback {
    pub id: Uuid,
    pub date: DateTime<Local>,
    pub user_id: Uuid,
    pub feedback: String,
}

#[Object]
impl Feedback {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    async fn user(&self, context: &Context<'_>) -> FieldResult<UserInfo> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .user_details(self.user_id)
            .await?
            .into())
    }

    async fn date(&self) -> DateTime<Utc> {
        self.date.into()
    }

    async fn feedback(&self) -> &str {
        &self.feedback
    }
}
