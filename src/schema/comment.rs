use crate::graphql::SchemaContext;
use async_graphql::*;
use chrono::prelude::*;
use divedb_core::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Dive, DiveQuery, PublicUserInfo};

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct DiveComment {
    pub id: Uuid,
    pub dive_id: Uuid,
    pub user_id: Uuid,
    pub description: String,
    pub date: DateTime<Local>,
    pub external: bool,
    pub ap_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct CreateDiveComment {
    pub id: Option<Uuid>,
    pub dive_id: Uuid,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject, Default)]
pub struct DiveCommentQuery {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub dive_id: Option<Uuid>,
}

#[Object]
impl DiveComment {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    async fn user(&self, context: &Context<'_>) -> FieldResult<PublicUserInfo> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .user_details(self.user_id)
            .await?
            .into())
    }

    async fn dive_id(&self) -> &Uuid {
        &self.dive_id
    }

    async fn dive(&self, context: &Context<'_>) -> FieldResult<Dive> {
        let context = context.data::<SchemaContext>()?;

        Ok(context
            .web
            .handle
            .dives(
                context.con.user.as_ref().map(|val| val.id),
                &DiveQuery {
                    id: Some(self.dive_id),
                    user_id: None,
                    username: None,
                    offset: None,
                    limit: Some(1),
                    dive_site: None,
                    max_depth: None,
                },
            )
            .await?
            .pop()
            .expect("There should always be a dive attached to a dive comment"))
    }

    async fn date(&self) -> DateTime<Utc> {
        self.date.into()
    }

    async fn description(&self) -> &str {
        &self.description
    }
}
