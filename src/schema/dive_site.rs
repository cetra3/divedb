use std::collections::HashMap;

use super::{Dive, DiveQuery, OgReference, OgReferenceQuery, Photo, PhotoQuery};
use crate::escape::{md_to_text, truncate};
use crate::{db::DbHandle, graphql::SchemaContext};
use async_graphql::*;
use async_trait::async_trait;
use chrono::prelude::*;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use divedb_core::FromRow;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tracing::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct DiveSiteBatcher {
    pub handle: DbHandle,
}

pub type DiveSiteLoader = Loader<Uuid, DiveSite, DiveSiteBatcher>;

impl DiveSiteBatcher {
    pub fn new(handle: &DbHandle) -> DiveSiteLoader {
        let batcher = DiveSiteBatcher {
            handle: handle.clone(),
        };

        Loader::new(batcher)
    }
}

#[async_trait]
impl BatchFn<Uuid, DiveSite> for DiveSiteBatcher {
    async fn load(&mut self, keys: &[Uuid]) -> HashMap<Uuid, DiveSite> {
        debug!("Got load for keys:{:?}", keys);

        let mut hashmap = HashMap::new();

        match self.handle.dive_sites_batch(keys).await {
            Ok(sites) => {
                for site in sites {
                    hashmap.insert(site.id, site);
                }
            }
            Err(err) => {
                error!("Error getting keys:{}", err);
            }
        }

        hashmap
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, SimpleObject)]
pub struct SiteMetric {
    pub photo_count: i32,
    pub dive_count: i32,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, ToSql, FromSql, Enum)]
pub enum Difficulty {
    OW,
    AOW,
    Tech,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct DiveSite {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub access: String,
    pub difficulty: Difficulty,
    pub depth: f64,
    pub lat: f64,
    pub lon: f64,
    pub published: bool,
    pub photo_id: Option<Uuid>,
    pub date: DateTime<Local>,
    pub slug: Option<String>,
}

#[Object]
impl DiveSite {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn user_id(&self) -> &Option<Uuid> {
        &self.user_id
    }

    async fn name(&self) -> &String {
        &self.name
    }

    async fn description(&self) -> &String {
        &self.description
    }

    async fn summary(&self) -> String {
        md_to_text(&truncate(&self.description, 250))
    }

    async fn access(&self) -> &String {
        &self.access
    }

    async fn difficulty(&self) -> &Difficulty {
        &self.difficulty
    }

    async fn depth(&self) -> &f64 {
        &self.depth
    }

    async fn lat(&self) -> &f64 {
        &self.lat
    }

    async fn lon(&self) -> &f64 {
        &self.lon
    }

    async fn published(&self) -> &bool {
        &self.published
    }

    async fn date(&self) -> &DateTime<Local> {
        &self.date
    }

    async fn slug(&self) -> &Option<String> {
        &self.slug
    }

    async fn site_metrics(&self, context: &Context<'_>) -> FieldResult<SiteMetric> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .site_metrics(self.id)
            .await?
            .unwrap_or_default())
    }

    async fn photo_id(&self) -> &Option<Uuid> {
        &self.photo_id
    }

    async fn photo(&self, context: &Context<'_>) -> FieldResult<Option<Photo>> {
        match self.photo_id {
            Some(val) => {
                let query = PhotoQuery::id(val);

                Ok(context
                    .data::<SchemaContext>()?
                    .web
                    .handle
                    .photos(None, &query)
                    .await?
                    .into_iter()
                    .next())
            }
            None => Ok(None),
        }
    }

    async fn latest_photos(&self, context: &Context<'_>) -> FieldResult<Vec<Photo>> {
        let query = PhotoQuery {
            dive_site: Some(self.id),
            ..Default::default()
        };

        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .photos(None, &query)
            .await?)
    }

    async fn latest_dives(&self, context: &Context<'_>) -> FieldResult<Vec<Dive>> {
        let query = DiveQuery {
            dive_site: Some(self.id),
            ..Default::default()
        };

        let context = context.data::<SchemaContext>()?;

        let user_id = context.con.user.as_ref().map(|val| val.id);

        Ok(context.web.handle.dives(user_id, &query).await?)
    }

    async fn references(&self, context: &Context<'_>) -> FieldResult<Vec<OgReference>> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .references(&OgReferenceQuery {
                dive_site_id: Some(self.id),
                ..Default::default()
            })
            .await?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct CreateDiveSite {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub access: String,
    pub difficulty: Difficulty,
    pub depth: f64,
    pub photo_id: Option<Uuid>,
    pub lat: f64,
    pub lon: f64,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject, Default)]
pub struct DiveSiteQuery {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub max_depth: Option<f64>,
}

impl DiveSiteQuery {
    pub fn id(id: Uuid) -> Self {
        DiveSiteQuery {
            id: Some(id),
            slug: None,
            name: None,
            max_depth: None,
        }
    }
}
