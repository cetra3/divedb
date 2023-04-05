use crate::graphql::SchemaContext;
use async_graphql::*;
use chrono::prelude::*;
use divedb_core::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{DiveSite, Photo, PhotoQuery};

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct DiveMetric {
    pub time: i32,
    pub depth: f32,
    pub pressure: Option<f32>,
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Dive {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Local>,
    pub duration: i32,
    pub depth: f32,
    pub dive_site_id: Option<Uuid>,
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

    async fn duration(&self) -> &i32 {
        &self.duration
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

    async fn number(&self, context: &Context<'_>) -> FieldResult<i64> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .dive_number(self.id, self.user_id)
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
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct CreateDive {
    pub id: Option<Uuid>,
    pub date: DateTime<Utc>,
    pub duration: i32,
    pub depth: f64,
    pub dive_site_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject, Default)]
pub struct DiveQuery {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub dive_site: Option<Uuid>,
    pub max_depth: Option<f64>,
}

impl DiveQuery {
    pub fn id(id: Uuid) -> Self {
        DiveQuery {
            id: Some(id),
            user_id: None,
            dive_site: None,
            max_depth: None,
        }
    }
}
