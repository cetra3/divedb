use std::sync::Arc;

use activitypub_federation::{
    actix_web::inbox::receive_activity,
    config::Data,
    fetch::webfinger::{Webfinger, WebfingerLink},
    protocol::context::WithContext,
    traits::Object,
    FEDERATION_CONTENT_TYPE,
};
use actix_web::{
    dev::HttpServiceFactory,
    error::{ErrorInternalServerError, ErrorNotFound},
    web::{self},
    Error, HttpRequest, HttpResponse,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, warn};
use url::Url;
use uuid::Uuid;

use crate::{
    schema::{activitypub::OrderedCollection, DiveQuery, User},
    WebContext, SITE_URL,
};

use self::activities::PersonAcceptedActivities;

pub mod activities;

pub fn configure_users() -> impl HttpServiceFactory {
    web::scope("/users")
        .route("{username}", web::get().to(get_user))
        .route("{username}/inbox", web::post().to(user_inbox))
        .route("{username}/outbox", web::get().to(user_outbox))
        .service(crate::frontend::frontend())
}

pub fn configure_dives() -> impl HttpServiceFactory {
    web::scope("/dives")
        .route("{dive_id}", web::get().to(get_dive))
        .service(crate::frontend::frontend())
}

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

pub async fn node_info_well_known() -> Result<HttpResponse, Error> {
    let node_info = NodeInfoWellKnown {
        links: vec![NodeInfoWellKnownLinks {
            rel: Url::parse("http://nodeinfo.diaspora.software/ns/schema/2.0").unwrap(),
            href: Url::parse(&format!("{}/nodeinfo/2.0.json", &*SITE_URL)).unwrap(),
        }],
    };
    Ok(HttpResponse::Ok().json(node_info))
}

pub async fn node_info() -> Result<HttpResponse, Error> {
    let protocols = Some(vec!["activitypub".to_string()]);
    // Since there are 3 registration options,
    // we need to set open_registrations as true if RegistrationMode is not Closed.
    let open_registrations = Some(true);
    let json = NodeInfo {
        version: Some("2.0".to_string()),
        software: Some(NodeInfoSoftware {
            name: Some("divedb".to_string()),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
        protocols,
        usage: None,
        open_registrations,
    };

    Ok(HttpResponse::Ok().json(json))
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfo {
    pub version: Option<String>,
    pub software: Option<NodeInfoSoftware>,
    pub protocols: Option<Vec<String>>,
    pub usage: Option<NodeInfoUsage>,
    pub open_registrations: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfoUsage {
    pub users: Option<NodeInfoUsers>,
    pub local_posts: Option<i64>,
    pub local_comments: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfoUsers {
    pub total: Option<i64>,
    pub active_halfyear: Option<i64>,
    pub active_month: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct NodeInfoSoftware {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct NodeInfoWellKnown {
    pub links: Vec<NodeInfoWellKnownLinks>,
}

#[derive(Serialize, Deserialize, Debug)]
struct NodeInfoWellKnownLinks {
    pub rel: Url,
    pub href: Url,
}

// Given `acct:cetra@divedb.net`
// Returns `cetra`

fn extract_webfinger_name<'a>(
    resource: &'a str,
    data: &Data<Arc<WebContext>>,
) -> Result<&'a str, Error> {
    if let Some(val) = resource
        .strip_suffix(data.domain())
        .and_then(|val| val.strip_suffix('@'))
        .and_then(|val| val.strip_prefix("acct:"))
    {
        return Ok(val);
    }

    Err(ErrorNotFound(format!("No resource found: {resource}")))
}

pub async fn webfinger(
    query: web::Query<WebfingerQuery>,
    data: Data<Arc<WebContext>>,
) -> Result<HttpResponse, Error> {
    let name = extract_webfinger_name(&query.resource, &data).map_err(ErrorNotFound)?;

    let user = data
        .handle
        .user_by_username(name)
        .await
        .map_err(|_| ErrorNotFound(format!("No resource found in db: {name}")))?;

    let apub_url = user.ap_id();

    let webfinger = Webfinger {
        subject: query.into_inner().resource,
        links: vec![
            WebfingerLink {
                rel: Some("http://webfinger.net/rel/profile-page".into()),
                kind: Some("text/html".into()),
                href: Some(apub_url.clone()),
                ..Default::default()
            },
            WebfingerLink {
                rel: Some("self".into()),
                kind: Some(FEDERATION_CONTENT_TYPE.into()),
                href: Some(apub_url),
                ..Default::default()
            },
        ],
        aliases: vec![],
        properties: Default::default(),
    };

    Ok(HttpResponse::Ok().json(webfinger))
}

pub async fn get_user(
    request: HttpRequest,
    username: web::Path<String>,
    data: Data<Arc<WebContext>>,
) -> Result<HttpResponse, Error> {
    if is_apub_request(&request) {
        let user = data
            .handle
            .user_by_username(&username)
            .await
            .map_err(ErrorNotFound)?;

        let person = user
            .into_json(&data)
            .await
            .map_err(ErrorInternalServerError)?;

        return Ok(HttpResponse::Ok()
            .content_type(FEDERATION_CONTENT_TYPE)
            .json(WithContext::new_default(person)));
    }

    crate::frontend::frontend_proxy(request, &data).await
}

pub async fn get_dive(
    request: HttpRequest,
    dive_id: web::Path<String>,
    data: Data<Arc<WebContext>>,
) -> Result<HttpResponse, Error> {
    if is_apub_request(&request) {
        let dive_id = Uuid::parse_str(&dive_id).map_err(ErrorInternalServerError)?;

        let dive = data
            .handle
            .dives(
                None,
                &DiveQuery {
                    id: Some(dive_id),
                    ..Default::default()
                },
            )
            .await
            .map_err(ErrorInternalServerError)?
            .pop()
            .ok_or_else(|| ErrorNotFound("No dive found"))?;

        let note = dive
            .into_json(&data)
            .await
            .map_err(ErrorInternalServerError)?;

        return Ok(HttpResponse::Ok()
            .content_type(FEDERATION_CONTENT_TYPE)
            .json(WithContext::new_default(note)));
    }

    crate::frontend::frontend_proxy(request, &data).await
}

pub fn is_apub_request(request: &HttpRequest) -> bool {
    if let Some(header) = request
        .headers()
        .get("accept")
        .and_then(|val| val.to_str().ok())
    {
        debug!("Accept Header: {header}");

        let lower_case = header.to_lowercase();
        if lower_case.contains(FEDERATION_CONTENT_TYPE)
            || lower_case.contains(ACTIVITYPUB_CONTENT_TYPE)
        {
            return true;
        }
    }

    false
}
/// See: https://www.w3.org/TR/activitypub/#retrieving-objects
pub const ACTIVITYPUB_CONTENT_TYPE: &str =
    "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"";

/// Handles messages received in user inbox
pub async fn user_inbox(
    request: HttpRequest,
    body: Bytes,
    data: Data<Arc<WebContext>>,
) -> Result<HttpResponse, Error> {
    let json_body: Value = serde_json::from_slice(&body)?;

    debug!("Inbox message received: {json_body}");

    receive_activity::<WithContext<PersonAcceptedActivities>, User, _>(request, body, &data)
        .await
        .map_err(|err| {
            warn!("Error with receiving activity: {err}");

            ErrorInternalServerError(err)
        })
}

pub async fn user_outbox(
    username: web::Path<String>,
    data: Data<Arc<WebContext>>,
) -> Result<HttpResponse, Error> {
    let user = data
        .handle
        .user_by_username(&username)
        .await
        .map_err(ErrorNotFound)?;

    let dives = data
        .handle
        .dives(
            None,
            &DiveQuery {
                user_id: Some(user.id),
                ..Default::default()
            },
        )
        .await
        .map_err(ErrorInternalServerError)?;

    let mut notes = Vec::with_capacity(dives.len());

    for dive in dives {
        notes.push(
            dive.into_json(&data)
                .await
                .map_err(ErrorInternalServerError)?,
        )
    }

    let collection = OrderedCollection {
        id: user
            .ap_outbox()
            .expect("Should always have an outbox for internal users"),
        kind: Default::default(),
        total_items: notes.len(),
        ordered_items: notes,
    };

    Ok(HttpResponse::Ok()
        .content_type(FEDERATION_CONTENT_TYPE)
        .json(WithContext::new_default(collection)))
}
