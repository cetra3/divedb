use std::collections::HashMap;

use async_graphql::*;
use divedb_core::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::SchemaContext;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}

#[Object]
impl Category {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn values(&self, context: &Context<'_>) -> FieldResult<Vec<CategoryValue>> {
        let context = context.data::<SchemaContext>()?;

        Ok(context.web.handle.category_values(Some(self.id)).await?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject, FromRow)]
pub struct CategoryValue {
    pub id: Uuid,
    pub category_id: Uuid,
    pub value: String,
}

pub type CategoryMap = HashMap<Uuid, Vec<Uuid>>;

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct CreateCategory {
    pub id: Option<Uuid>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct CreateCategoryValue {
    pub id: Option<Uuid>,
    pub category_id: Uuid,
    pub value: String,
}
