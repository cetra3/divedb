use actix_web::{
    dev::HttpServiceFactory,
    error::ErrorBadRequest,
    http::Method,
    web::{self, Bytes},
    Error, HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};
use tracing::*;

use crate::{cache_header, graphql::WebContext};

#[derive(Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    bytes: Bytes,
    mime_type: String,
}

pub fn frontend() -> impl HttpServiceFactory {
    web::scope("")
        .wrap(cache_header(86400))
        .default_service(web::route().to(frontend_route))
}

async fn frontend_route(
    req: HttpRequest,
    web: web::Data<WebContext>,
) -> Result<HttpResponse, Error> {
    frontend_proxy(req, &web).await
}

pub async fn frontend_proxy(req: HttpRequest, web: &WebContext) -> Result<HttpResponse, Error> {
    // Only support GET requests for now
    if req.method() != Method::GET {
        return Ok(HttpResponse::MethodNotAllowed()
            .insert_header(("Allow", "GET"))
            .body("Only GET requests are supported"));
    }

    let path = req.path().to_string();
    let query_string = req.query_string().to_string();

    let relative_path = if query_string.is_empty() {
        path.clone()
    } else {
        format!("{}?{}", path, query_string)
    };

    let target_url = format!("{}{relative_path}", web.frontend_url);

    let client = web.client.clone();
    // Use fetch method for cache-aside pattern
    let cached_entry = web
        .frontend_cache
        .fetch(relative_path.clone(), {
            || async move {
                debug!("Proxying request to: {}", target_url);

                let response = client
                    .get(target_url)
                    .send()
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                // get mime type and send in response
                let mime_type = response
                    .headers()
                    .get("content-type")
                    .and_then(|mime| mime.to_str().ok())
                    .unwrap_or("application/octet-stream")
                    .to_string();

                let response_bytes = response
                    .bytes()
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                // Return the cache entry
                Ok(CacheEntry {
                    bytes: response_bytes,
                    mime_type,
                })
            }
        })
        .await
        .map_err(ErrorBadRequest)?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", cached_entry.value().mime_type.clone()))
        .body(cached_entry.value().bytes.clone()))
}
