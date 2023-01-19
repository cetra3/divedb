use anyhow::{anyhow, Error};
use async_graphql::*;
use chrono::prelude::*;
use divedb_core::FromRow;
use log::debug;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use once_cell::sync::Lazy;

use crate::escape::md_to_text;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
pub struct OgReference {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub image_url: Option<String>,
    pub description: String,
    pub last_fetched: DateTime<Local>,
}

static OG_TITLE: Lazy<Selector> =
    Lazy::new(|| Selector::parse("meta[property=\"og:title\"]").unwrap());

static TITLE: Lazy<Selector> = Lazy::new(|| Selector::parse("title").unwrap());

static DESCRIPTION: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("meta[property=\"og:description\"], meta[name=\"description\"]").unwrap()
});

static IMAGE: Lazy<Selector> =
    Lazy::new(|| Selector::parse("meta[property=\"og:image\"]").unwrap());

impl OgReference {
    pub async fn from_url(client: &Client, url: &str) -> Result<Self, Error> {
        let response = client.get(url).send().await?.text().await?;

        debug!("Response:{response}");

        let document = Html::parse_document(&response);

        let title = get_content(&document, &OG_TITLE)
            .map(String::from)
            .or_else(|| document.select(&TITLE).next().map(|val| val.inner_html()))
            .ok_or_else(|| anyhow!("No title found for website"))?;
        let description = get_content(&document, &DESCRIPTION).unwrap_or_default();
        let image_url = get_content(&document, &IMAGE).map(String::from);

        Ok(OgReference {
            id: Uuid::new_v4(),
            url: url.to_string(),
            title: md_to_text(&title),
            image_url,
            description: md_to_text(description),
            last_fetched: Local::now(),
        })
    }
}

fn get_content<'a>(document: &'a Html, selector: &'a Selector) -> Option<&'a str> {
    document
        .select(selector)
        .next()
        .and_then(|val| val.value().attr("content"))
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, InputObject)]
pub struct CreateOgReference {
    pub url: String,
    pub sealife_id: Option<Uuid>,
    pub dive_site_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, InputObject)]
pub struct OgReferenceQuery {
    pub id: Option<Uuid>,
    pub url: Option<String>,
    pub dive_site_id: Option<Uuid>,
    pub sealife_id: Option<Uuid>,
}
