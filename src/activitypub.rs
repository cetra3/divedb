use std::sync::Arc;

use activitypub_federation::{
    actix_web::inbox::receive_activity,
    config::Data,
    fetch::webfinger::{extract_webfinger_name, Webfinger, WebfingerLink},
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
use log::{debug, warn};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    schema::{activitypub::OrderedCollection, DiveQuery, User},
    WebContext, SITE_URL,
};

use self::activities::PersonAcceptedActivities;

pub mod activities;

pub fn configure() -> impl HttpServiceFactory {
    web::scope("/users")
        .route("{username}", web::get().to(get_user))
        .route("{username}/inbox", web::post().to(user_inbox))
        .route("{username}/outbox", web::get().to(user_outbox))
}

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

pub async fn webfinger(
    query: web::Query<WebfingerQuery>,
    data: Data<Arc<WebContext>>,
) -> Result<HttpResponse, Error> {
    let name = extract_webfinger_name(&query.resource, &data).map_err(ErrorNotFound)?;

    let user = data
        .handle
        .user_by_username(&name)
        .await
        .map_err(ErrorNotFound)?;

    let profile_url = format!("{}/dives?u={}", &*SITE_URL, user.username)
        .parse()
        .map_err(ErrorInternalServerError)?;

    let apub_url = user.ap_id();

    let webfinger = Webfinger {
        subject: query.into_inner().resource,
        links: vec![
            WebfingerLink {
                rel: Some("http://webfinger.net/rel/profile-page".into()),
                kind: Some("text/html".into()),
                href: Some(profile_url),
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
    username: web::Path<String>,
    data: Data<Arc<WebContext>>,
) -> Result<HttpResponse, Error> {
    let user = data
        .handle
        .user_by_username(&username)
        .await
        .map_err(ErrorNotFound)?;

    let person = user
        .into_json(&data)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(FEDERATION_CONTENT_TYPE)
        .json(WithContext::new_default(person)))
}

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
