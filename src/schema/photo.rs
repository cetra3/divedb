use super::{Dive, DiveQuery, DiveSite, PublicUserInfo, Sealife, SealifeQuery};
use crate::graphql::SchemaContext;
use async_graphql::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use divedb_core::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Photo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub date: Option<DateTime<Local>>,
    pub dive_id: Option<Uuid>,
    pub dive_site_id: Option<Uuid>,
    pub size: i32,
    pub upload_date: DateTime<Local>,
    pub hide_location: Option<bool>,
    pub width: i32,
    pub height: i32,
    pub description: String,
    pub copyright: Option<String>,
    pub internal: bool,
}

impl Photo {
    fn uuid_folder(&self) -> String {
        let uuid_string = self.id.to_string();
        format!(
            "{}/{}/{}",
            &uuid_string[0..2],
            &uuid_string[2..4],
            &uuid_string[4..]
        )
    }

    pub fn orig_location(&self) -> String {
        format!("store/{}/{}", self.uuid_folder(), self.filename)
    }

    fn thumb_prefix(&self) -> String {
        format!("thumbs/{}", self.uuid_folder())
    }

    fn large_prefix(&self) -> String {
        format!("large/{}", self.uuid_folder())
    }

    pub fn webp_thumb_location(&self) -> String {
        let ext_start = self.filename.rfind('.').unwrap_or(self.filename.len());

        format!(
            "{}/{}.webp",
            self.thumb_prefix(),
            &self.filename[0..ext_start]
        )
    }

    pub fn webp_large_location(&self) -> String {
        let ext_start = self.filename.rfind('.').unwrap_or(self.filename.len());

        format!(
            "{}/{}.webp",
            self.large_prefix(),
            &self.filename[0..ext_start]
        )
    }

    pub fn jpg_thumb_location(&self) -> String {
        format!("{}/{}", self.thumb_prefix(), self.filename)
    }

    pub fn jpg_large_location(&self) -> String {
        format!("{}/{}", self.large_prefix(), self.filename)
    }
}

#[Object]
impl Photo {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    async fn date(&self) -> Option<DateTime<Utc>> {
        self.date.map(|val| val.into())
    }

    async fn filename(&self) -> &str {
        &self.filename
    }

    async fn size(&self) -> f64 {
        self.size as f64
    }

    async fn width(&self) -> f64 {
        self.width as f64
    }

    async fn height(&self) -> f64 {
        self.height as f64
    }

    async fn dive(&self, context: &Context<'_>) -> FieldResult<Option<Dive>> {
        let context = context.data::<SchemaContext>()?;

        if let Some(id) = self.dive_id {
            Ok(context
                .web
                .handle
                .dives(
                    context.con.user.as_ref().map(|val| val.id),
                    &DiveQuery {
                        id: Some(id),
                        user_id: None,
                        username: None,
                        dive_site: None,
                        max_depth: None,
                        offset: None,
                        limit: Some(1),
                    },
                )
                .await?
                .pop())
        } else {
            Ok(None)
        }
    }

    async fn sealife(&self, context: &Context<'_>) -> FieldResult<Option<Sealife>> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .sealife(&SealifeQuery {
                photo_id: Some(self.id),
                ..Default::default()
            })
            .await?
            .pop())
    }

    async fn dive_site(&self, context: &Context<'_>) -> FieldResult<Option<DiveSite>> {
        let mut hide_location = self.hide_location.unwrap_or_default();

        let context = context.data::<SchemaContext>()?;

        let mut editor = false;

        if let Some(user) = context.con.user.as_ref() {
            if user.is_editor() || user.id == self.user_id {
                hide_location = false;
            }

            editor = user.is_editor();
        }

        if hide_location {
            return Ok(None);
        }

        if let Some(id) = self.dive_site_id {
            let dive_site = context.web.dive_batch.load(id).await;

            if dive_site.published
                || editor
                || dive_site.user_id == context.con.user.as_ref().map(|user| user.id)
            {
                return Ok(Some(dive_site));
            }
        }
        Ok(None)
    }

    async fn likes(&self, context: &Context<'_>) -> FieldResult<i64> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .photo_likes(self.id)
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
                .photo_liked(user_id, self.id)
                .await?)
        } else {
            Ok(false)
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
pub struct CreatePhoto {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub filename: String,
    pub date: DateTime<Local>,
    pub dive_id: Option<Uuid>,
    pub size: i32,
    pub internal: Option<bool>,
    pub dive_site_id: Option<Uuid>,
    pub sealife_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject, Default)]
pub struct PhotoQuery {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub dive_site: Option<Uuid>,
    pub dive: Option<Uuid>,
    pub sealife_id: Option<Uuid>,
    pub order_by_upload: Option<bool>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl PhotoQuery {
    pub fn id(id: Uuid) -> Self {
        PhotoQuery {
            id: Some(id),
            user_id: None,
            username: None,
            dive_site: None,
            dive: None,
            sealife_id: None,
            offset: None,
            order_by_upload: None,
            limit: Some(10),
        }
    }
}
