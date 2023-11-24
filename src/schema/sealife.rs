use std::collections::HashMap;

use super::{OgReference, OgReferenceQuery, Photo, PhotoQuery};
use crate::escape::{md_to_text, truncate};
use crate::{db::DbHandle, graphql::SchemaContext};
use async_graphql::*;
use async_trait::async_trait;
use chrono::prelude::*;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use divedb_core::FromRow;
use serde::{Deserialize, Serialize};
use tracing::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct SealifeBatcher {
    pub handle: DbHandle,
}

pub type SealifeLoader = Loader<Uuid, Sealife, SealifeBatcher>;

impl SealifeBatcher {
    pub fn new(handle: &DbHandle) -> SealifeLoader {
        let batcher = SealifeBatcher {
            handle: handle.clone(),
        };

        Loader::new(batcher)
    }
}

#[async_trait]
impl BatchFn<Uuid, Sealife> for SealifeBatcher {
    async fn load(&mut self, keys: &[Uuid]) -> HashMap<Uuid, Sealife> {
        debug!("Got load for keys:{:?}", keys);

        let mut hashmap = HashMap::new();

        match self.handle.sealife_batch(keys).await {
            Ok(sealife) => {
                for sealife in sealife {
                    hashmap.insert(sealife.id, sealife);
                }
            }
            Err(err) => {
                error!("Error getting keys:{}", err);
            }
        }

        hashmap
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Sealife {
    pub id: Uuid,
    pub name: String,
    pub scientific_name: Option<String>,
    pub description: String,
    pub photo_id: Option<Uuid>,
    pub date: DateTime<Local>,
    pub slug: Option<String>,
    pub hide_location: bool,
}

#[Object]
impl Sealife {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn name(&self) -> &String {
        &self.name
    }

    async fn scientific_name(&self) -> &Option<String> {
        &self.scientific_name
    }

    async fn description(&self) -> &String {
        &self.description
    }

    async fn date(&self) -> DateTime<Local> {
        self.date
    }

    async fn summary(&self) -> String {
        md_to_text(&truncate(&self.description, 250))
    }

    async fn photo_id(&self) -> &Option<Uuid> {
        &self.photo_id
    }

    async fn photo(&self, context: &Context<'_>) -> FieldResult<Option<Photo>> {
        if let Some(id) = self.photo_id {
            Ok(context
                .data::<SchemaContext>()?
                .web
                .handle
                .photos(None, &PhotoQuery::id(id))
                .await?
                .pop())
        } else {
            Ok(None)
        }
    }

    async fn latest_photos(&self, context: &Context<'_>) -> FieldResult<Vec<Photo>> {
        let query = PhotoQuery {
            sealife_id: Some(self.id),
            ..Default::default()
        };

        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .photos(None, &query)
            .await?)
    }

    async fn category_map(&self, context: &Context<'_>) -> FieldResult<HashMap<String, Vec<Uuid>>> {
        let mut string_map: HashMap<String, Vec<Uuid>> = HashMap::new();

        let category_map = context
            .data::<SchemaContext>()?
            .web
            .handle
            .category_map(self.id)
            .await?;

        for (key, val) in category_map {
            string_map.insert(key.to_string(), val);
        }

        Ok(string_map)
    }

    async fn slug(&self) -> &Option<String> {
        &self.slug
    }

    async fn references(&self, context: &Context<'_>) -> FieldResult<Vec<OgReference>> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .references(&OgReferenceQuery {
                sealife_id: Some(self.id),
                ..Default::default()
            })
            .await?)
    }

    async fn hide_location(&self) -> &bool {
        &self.hide_location
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct CreateSealife {
    pub id: Option<Uuid>,
    pub name: String,
    pub scientific_name: Option<String>,
    pub description: String,
    pub photo_id: Option<Uuid>,
    pub category_map: Option<HashMap<String, Vec<Uuid>>>,
    pub hide_location: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject, Default)]
pub struct SealifeQuery {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub scientific_name: Option<String>,
    pub slug: Option<String>,
    pub photo_id: Option<Uuid>,
    pub category_values: Option<Vec<Uuid>>,
}

impl SealifeQuery {
    pub fn id(id: Uuid) -> Self {
        SealifeQuery {
            id: Some(id),
            name: None,
            scientific_name: None,
            slug: None,
            photo_id: None,
            category_values: None,
        }
    }
}
