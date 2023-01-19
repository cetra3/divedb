use async_graphql::*;
use divedb_core::FromRow;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct LoginResponse {
    pub id: Uuid,
    pub email: String,
    pub token: String,
    pub level: UserLevel,
    pub username: Option<String>,
    pub watermark_location: OverlayLocation,
    pub copyright_location: Option<OverlayLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub level: UserLevel,
    pub username: Option<String>,
    pub watermark_location: OverlayLocation,
    pub copyright_location: Option<OverlayLocation>,
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
