use chrono::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{
        actor::PersonType,
        collection::OrderedCollectionType,
        object::{ImageType, NoteType},
    },
    protocol::{
        helpers::deserialize_one_or_many, public_key::PublicKey, verification::verify_domains_match,
    },
    traits::{Actor, Object},
};

use anyhow::{anyhow, Error};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{escape::md_to_html, WebContext, SITE_URL};

use super::{Dive, DiveComment, DiveSite, DiveSiteQuery, Photo, PhotoQuery, User};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    id: ObjectId<User>,
    inbox: Url,
    outbox: Option<Url>,
    public_key: PublicKey,
}

impl User {
    pub fn ap_id(&self) -> Url {
        self.ap_id
            .as_ref()
            .map(|val| val.parse::<Url>().expect("Invalid Site Url!"))
            .unwrap_or_else(|| {
                format!("{}/users/{}", &*SITE_URL, self.username)
                    .parse::<Url>()
                    .expect("Invalid Site Url!")
            })
    }

    pub fn ap_inbox(&self) -> Url {
        self.inbox
            .as_ref()
            .map(|val| val.parse::<Url>().expect("Invalid Site Url!"))
            .unwrap_or_else(|| {
                format!("{}/users/{}/inbox", &*SITE_URL, self.username)
                    .parse::<Url>()
                    .expect("Invalid Site Url!")
            })
    }

    pub fn ap_outbox(&self) -> Option<Url> {
        if self.external {
            None
        } else {
            Some(
                format!("{}/users/{}/outbox", &*SITE_URL, self.username)
                    .parse::<Url>()
                    .expect("Invalid Site Url!"),
            )
        }
    }
}

impl Actor for User {
    fn id(&self) -> Url {
        self.ap_id()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }

    fn inbox(&self) -> Url {
        self.ap_inbox()
    }
}

#[async_trait::async_trait]
impl Object for User {
    type DataType = Arc<WebContext>;
    type Kind = Person;
    type Error = Error;

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let prefix = format!("{}/users/", &*SITE_URL);

        if let Some(username) = object_id.to_string().strip_prefix(&prefix) {
            return Ok(data.handle.user_by_username(username).await.ok());
        }

        Ok(data.handle.user_by_ap_id(object_id.as_str()).await.ok())
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        let apub_id = self.ap_id();

        let inbox = self.ap_inbox();
        let outbox = self.ap_outbox();

        let public_key = self.public_key();

        let preferred_username = self.username;

        let name = self.display_name;

        Ok(Person {
            preferred_username,
            kind: Default::default(),
            id: apub_id.into(),
            name,
            inbox,
            outbox,
            public_key,
        })
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        if json
            .id
            .inner()
            .domain()
            .ok_or_else(|| anyhow!("Invalid ID"))?
            == data.domain()
        {
            return Self::read_from_id(json.id.into(), data)
                .await?
                .ok_or_else(|| anyhow!("No user found!"));
        } else {
            if let Ok(user) = data.handle.user_by_ap_id(json.id.inner().as_str()).await {
                return Ok(user);
            } else {
                return data
                    .handle
                    .new_external_user(
                        &json.preferred_username,
                        json.id.inner().as_str(),
                        json.inbox.as_str(),
                        &json.public_key.public_key_pem,
                    )
                    .await;
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: ObjectId<Dive>,
    #[serde(rename = "type")]
    pub kind: NoteType,
    pub attributed_to: ObjectId<User>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub to: Vec<Url>,
    pub content: String,
    pub summary: Option<String>,
    pub published: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub tag: Vec<Tag>,
    pub attachment: Vec<Image>,
}

#[async_trait::async_trait]
impl Object for Dive {
    type DataType = Arc<WebContext>;
    type Kind = Note;
    type Error = Error;

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let prefix = format!("{}/dives/", &*SITE_URL);

        if let Some(id) = object_id
            .as_str()
            .strip_prefix(&prefix)
            .and_then(|val| Uuid::parse_str(val).ok())
        {
            return Ok(data
                .handle
                .dives(
                    None,
                    &super::DiveQuery {
                        id: Some(id),
                        ..Default::default()
                    },
                )
                .await
                .ok()
                .and_then(|mut val| val.pop()));
        }

        Ok(None)
    }

    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        let id = format!("{}/dives/{}", &*SITE_URL, self.id)
            .parse::<Url>()?
            .into();

        let attributed_to = data.handle.user_details(self.user_id).await?.ap_id().into();

        let to = if self.published {
            vec!["https://www.w3.org/ns/activitystreams#Public"
                .parse::<Url>()
                .unwrap()]
        } else {
            vec![]
        };

        let mut dive_site = None;

        if let Some(site_id) = self.dive_site_id {
            dive_site = data
                .handle
                .dive_sites(
                    None,
                    &DiveSiteQuery {
                        id: Some(site_id),
                        ..Default::default()
                    },
                )
                .await?
                .pop()
        }

        let mut content_builder = format!("**Dive #{}", self.dive_number);

        if let Some(ref site) = dive_site {
            content_builder.push_str(" - ");
            content_builder.push_str(&site.name);
        }
        content_builder.push_str("**\n");
        content_builder.push_str(&self.description);

        let content = md_to_html(&content_builder);
        let published = self.date.with_timezone(&Utc);

        let mut attachment = data
            .handle
            .photos(
                None,
                &PhotoQuery {
                    dive: Some(self.id),
                    ..Default::default()
                },
            )
            .await?
            .into_iter()
            .map(|photo| Image::from_photo(&photo))
            .collect::<Vec<_>>();

        // Put the dive chart at the end of the photos
        if data.handle.has_metrics(self.id).await? {
            attachment.push(Image::dive_chart(self.id));
        }

        let tag = if let Some(ref site) = dive_site {
            vec![Tag::from_site(&site)]
        } else {
            vec![]
        };

        Ok(Note {
            id,
            kind: NoteType::Note,
            attributed_to,
            to,
            content,
            summary: None,
            published,
            updated: Utc::now(),
            tag,
            attachment,
        })
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        Self::read_from_id(json.id.into(), data)
            .await?
            .ok_or_else(|| anyhow!("No dive found!"))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteReply {
    pub id: ObjectId<DiveComment>,
    #[serde(rename = "type")]
    pub kind: NoteType,
    pub in_reply_to: ObjectId<Dive>,
    pub attributed_to: ObjectId<User>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub to: Vec<Url>,
    pub content: String,
    pub published: DateTime<Utc>,
}

#[async_trait::async_trait]
impl Object for DiveComment {
    type DataType = Arc<WebContext>;
    type Kind = NoteReply;
    type Error = Error;

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        unimplemented!();
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        unimplemented!()
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    kind: ImageType,
    media_type: String,
    url: String,
    name: Option<String>,
    width: i32,
    height: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(rename = "type")]
    kind: String,
    href: String,
    name: String,
}

impl Tag {
    pub fn from_site(site: &DiveSite) -> Self {
        let slug = site.slug.as_deref().unwrap_or_default();
        Self {
            kind: "Hashtag".into(),
            href: format!("{}/sites/{}", &*SITE_URL, slug),
            name: format!("#{}", slug.replace("-", "_")),
        }
    }
}

impl Image {
    pub fn dive_chart(dive_id: Uuid) -> Self {
        let url = format!("{}/api/chart/{}/png", &*SITE_URL, dive_id);
        let media_type = "image/png".to_string();
        let width = 877;
        let height = 300;

        Image {
            kind: ImageType::Image,
            media_type,
            url,
            name: None,
            width,
            height,
        }
    }

    pub fn from_photo(photo: &Photo) -> Self {
        let kind = ImageType::Image;
        let media_type = "image/jpeg".to_string();

        let url = format!("{}/api/photos/jpeglarge/{}", &*SITE_URL, photo.id);

        // Large size
        let desired_width = 2000;
        let width;
        let height;

        if photo.width > photo.height {
            let ratio = photo.height as f32 / photo.width as f32;
            width = desired_width;
            height = (desired_width as f32 * ratio) as i32;
        } else {
            let ratio = photo.width as f32 / photo.height as f32;
            width = (desired_width as f32 * ratio) as i32;
            height = desired_width;
        }

        Image {
            kind,
            media_type,
            url,
            name: None,
            width,
            height,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    pub id: Url,
    #[serde(rename = "type")]
    pub kind: OrderedCollectionType,
    pub total_items: usize,
    pub ordered_items: Vec<Note>,
}
