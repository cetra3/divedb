use async_graphql::*;
use chrono::prelude::*;
use divedb_core::FromRow;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::SchemaContext;

use super::{Dive, DiveQuery, Photo, PhotoQuery};

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct LoginResponse {
    pub id: Uuid,
    pub email: String,
    pub token: String,
    pub level: UserLevel,
    pub username: String,
    pub display_name: Option<String>,
    pub watermark_location: OverlayLocation,
    pub copyright_location: Option<OverlayLocation>,
    pub description: String,
    pub photo_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: Option<String>,
    pub level: UserLevel,
    pub username: String,
    pub display_name: Option<String>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicUserInfo {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub description: String,
    pub photo_id: Option<Uuid>,
}

#[Object]
impl PublicUserInfo {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn display_name(&self) -> &Option<String> {
        &self.display_name
    }

    async fn description(&self) -> &str {
        &self.description
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

    async fn photo_count(&self, context: &Context<'_>) -> FieldResult<i64> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .photo_count(self.id)
            .await?)
    }

    async fn dive_count(&self, context: &Context<'_>) -> FieldResult<i64> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .dive_count(self.id)
            .await?)
    }

    async fn latest_dives(&self, context: &Context<'_>) -> FieldResult<Vec<Dive>> {
        let query = DiveQuery {
            user_id: Some(self.id),
            limit: Some(4),
            ..Default::default()
        };

        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .dives(None, &query)
            .await?)
    }

    async fn latest_photos(&self, context: &Context<'_>) -> FieldResult<Vec<Photo>> {
        let query = PhotoQuery {
            user_id: Some(self.id),
            limit: Some(4),
            ..Default::default()
        };

        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .photos(None, &query)
            .await?)
    }
}

impl From<User> for PublicUserInfo {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            display_name: value.display_name,
            description: value.description,
            photo_id: value.photo_id,
        }
    }
}

impl From<User> for UserInfo {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            level: value.level,
            username: value.username,
            display_name: value.display_name,
            description: value.description,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub password: Option<String>,
    pub level: UserLevel,
    pub username: String,
    pub watermark_location: OverlayLocation,
    pub copyright_location: Option<OverlayLocation>,
    pub email_verified: bool,
    pub display_name: Option<String>,
    pub public_key: String,
    pub private_key: Option<String>,
    pub description: String,
    pub date: DateTime<Local>,
    pub photo_id: Option<Uuid>,
    pub external: bool,
    pub ap_id: Option<String>,
    pub inbox: Option<String>,
}

impl User {
    pub fn is_editor(&self) -> bool {
        match self.level {
            UserLevel::User => false,
            UserLevel::Editor => true,
            UserLevel::Admin => true,
        }
    }

    pub fn is_admin(&self) -> bool {
        match self.level {
            UserLevel::User => false,
            UserLevel::Editor => false,
            UserLevel::Admin => true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, ToSql, FromSql, Enum)]
pub enum UserLevel {
    User,
    Editor,
    Admin,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, ToSql, FromSql, Enum)]
pub enum OverlayLocation {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
