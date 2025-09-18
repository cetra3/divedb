use crate::{
    escape::{md_to_text, truncate},
    graphql::SchemaContext,
};
use async_graphql::*;
use chrono::prelude::*;
use divedb_core::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{DiveComment, DiveCommentQuery, DiveSite, Photo, PhotoQuery, PublicUserInfo};

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject, FromRow)]
pub struct DiveMetric {
    pub time: i32,
    pub depth: f32,
    pub pressure: Option<f32>,
    pub temperature: Option<f32>,
    pub o2: Option<f32>,
    pub he: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, FromRow)]
pub struct Dive {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Local>,
    pub duration: i32,
    pub depth: f32,
    pub dive_site_id: Option<Uuid>,
    pub dive_number: i32,
    pub description: String,
    pub published: bool,
    pub deco_model: Option<String>,
}

#[Object]
impl Dive {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    async fn date(&self) -> DateTime<Utc> {
        self.date.into()
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn summary(&self) -> String {
        md_to_text(&truncate(&self.description, 250))
    }

    async fn duration(&self) -> i32 {
        self.duration
    }

    async fn depth(&self) -> f64 {
        self.depth as f64
    }

    async fn latest_photos(&self, context: &Context<'_>) -> FieldResult<Vec<Photo>> {
        let query = PhotoQuery {
            dive: Some(self.id),
            ..Default::default()
        };

        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .photos(None, &query)
            .await?)
    }

    async fn number(&self) -> i32 {
        self.dive_number
    }

    async fn published(&self) -> bool {
        self.published
    }

    async fn likes(&self, context: &Context<'_>) -> FieldResult<i64> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .dive_likes(self.id)
            .await?)
    }

    async fn liked(&self, context: &Context<'_>) -> FieldResult<bool> {
        if let Some(user_id) = context
            .data::<SchemaContext>()?
            .con
            .user
            .as_ref()
            .map(|val| val.id)
        {
            Ok(context
                .data::<SchemaContext>()?
                .web
                .handle
                .dive_liked(user_id, self.id)
                .await?)
        } else {
            Ok(false)
        }
    }

    async fn num_comments(&self, context: &Context<'_>) -> FieldResult<i64> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .dive_comments(self.id)
            .await?)
    }

    async fn comments(&self, context: &Context<'_>) -> FieldResult<Vec<DiveComment>> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .comments(&DiveCommentQuery {
                id: None,
                user_id: None,
                dive_id: Some(self.id),
            })
            .await?)
    }

    async fn has_metrics(&self, context: &Context<'_>) -> FieldResult<bool> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .has_metrics(self.id)
            .await?)
    }
    async fn dive_site_id(&self) -> Option<Uuid> {
        self.dive_site_id
    }

    async fn dive_site(&self, context: &Context<'_>) -> FieldResult<Option<DiveSite>> {
        if let Some(id) = self.dive_site_id {
            Ok(Some(
                context
                    .data::<SchemaContext>()?
                    .web
                    .dive_batch
                    .load(id)
                    .await,
            ))
        } else {
            Ok(None)
        }
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
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct CreateDive {
    pub id: Option<Uuid>,
    pub date: DateTime<Utc>,
    pub duration: i32,
    pub depth: f64,
    pub dive_site_id: Option<Uuid>,
    pub description: String,
    pub published: bool,
    pub deco_model: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject, Default)]
pub struct DiveQuery {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub dive_site: Option<Uuid>,
    pub max_depth: Option<f64>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl DiveQuery {
    pub fn id(id: Uuid) -> Self {
        DiveQuery {
            id: Some(id),
            user_id: None,
            username: None,
            dive_site: None,
            max_depth: None,
            offset: None,
            limit: None,
        }
    }
}
