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
    pub username: String,
    pub display_name: Option<String>,
    pub watermark_location: OverlayLocation,
    pub copyright_location: Option<OverlayLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: Option<String>,
    pub level: UserLevel,
    pub username: String,
    pub display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct PublicUserInfo {
    pub id: Uuid,
    pub level: UserLevel,
    pub username: String,
    pub display_name: Option<String>,
}

impl From<User> for PublicUserInfo {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            level: value.level,
            username: value.username,
            display_name: value.display_name,
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
