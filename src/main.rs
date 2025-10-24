use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use actix_web::{
    dev::ConnectionInfo,
    error::ErrorInternalServerError,
    middleware::{self, DefaultHeaders},
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer,
};

use anyhow::{anyhow, Error};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    CacheControl,
};
use clap::Parser;
use facebook::FacebookOauth;
use foyer::{DirectFsDeviceOptions, Engine, HybridCacheBuilder, LargeEngineOptions};
use once_cell::sync::Lazy;
use photos::{open_photo, save_files};
use rand::{thread_rng, Rng};
use std::{
    cmp, env,
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
    sync::Arc,
};
use token::Token;
use tracing::*;
use tracing_subscriber::{prelude::*, EnvFilter};
use url::Url;
mod activitypub;
pub mod db;
// mod photos;
pub mod chart;
pub mod email;
pub mod escape;
pub mod facebook;
pub mod frontend;
pub mod graphql;
pub mod openid;
mod photos;
pub mod plan;
pub mod schema;
pub mod search;
mod seo;
pub mod subsurface;
pub mod token;

use db::DbHandle;
use graphql::*;
use schema::User;

use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::{
    chart::{png_chart, svg_chart},
    email::Emailer,
    openid::{OpenIDClient, OpenIDSettings},
    photos::{image_dims, resize_image},
    schema::{DiveSiteBatcher, Photo, PhotoQuery, SealifeBatcher},
    search::Searcher,
    seo::{robots, sitemap_handler},
};

// Sets up the postgres connection as given by `-c`
#[derive(Parser, Clone, Debug, PartialEq)]
#[command(name = "divedb", about = "divedb Demo")]
pub struct ConfigContext {
    #[arg(
        short = 'c',
        long = "connect_url",
        help = "PostgreSQL Connection URL",
        default_value = "postgres://divedb:divedb@localhost",
        env
    )]
    connect_url: String,

    #[arg(short = 'w', help = "Write out schema to schema.grapqhl", long)]
    write_schema: bool,

    #[arg(short = 'r', help = "Refresh all thumbnails/photos", long)]
    refresh_photos: bool,

    #[arg(short = 'l', help = "Listen Address", default_value = "[::]:3333", env)]
    listen_address: String,

    #[arg(long, default_value = "cache", env)]
    cache_dir: String,

    #[arg(long, default_value = "http://localhost:3000", env)]
    frontend_url: String,

    #[arg(long, env)]
    admin_email: Option<String>,

    #[command(flatten)]
    facebook: FacebookOauth,

    #[command(flatten)]
    site_context: SiteContext,

    #[command(flatten)]
    email_settings: EmailSettings,

    #[command(flatten)]
    open_id_settings: Option<OpenIDSettings>,
}

#[derive(Parser, Clone, Debug, PartialEq)]
pub struct SiteContext {
    #[arg(long, env)]
    pub secret_key: Option<String>,
}

pub static SITE_URL: Lazy<String> =
    Lazy::new(|| std::env::var("SITE_URL").unwrap_or("https://divedb.net".into()));

#[derive(Parser, Clone, Debug, PartialEq)]
pub struct EmailSettings {
    #[arg(long, env, default_value = "localhost")]
    pub smtp_host: String,
    #[arg(long, env, default_value = "25")]
    pub smtp_port: u16,
    #[arg(long, env, default_value = "contact@divedb.net")]
    pub from_addr: String,
    #[arg(long, env, default_value = "tls")]
    pub smtp_security: EmailSecurity,
    #[arg(long, env)]
    pub smtp_username: Option<String>,
    #[arg(long, env)]
    pub smtp_password: Option<String>,
}

impl FromStr for EmailSecurity {
    type Err = Error;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "none" => Ok(EmailSecurity::None),
            _ => Ok(EmailSecurity::Tls),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EmailSecurity {
    None,
    Tls,
}

impl SiteContext {
    fn get_key(&self) -> [u8; 32] {
        let mut arr = [0u8; 32];

        if let Some(ref string_key) = self.secret_key {
            let secret_key = string_key.as_bytes();

            let len = cmp::min(32, secret_key.len());

            if len != 32 {
                warn!(
                    "Secret key length is {}, please ensure your key length is 32 characters",
                    len
                )
            }

            arr[..len].copy_from_slice(&secret_key[..len]);
        } else {
            warn!("No `SECRET_KEY` var set, using a random key.  Users will be logged out when the service restarts");
            thread_rng().fill(&mut arr);
        }

        arr
    }
}

pub fn log_error<E: Into<Error>>(err: E) -> actix_web::Error {
    let err: Error = err.into();
    error!("{:?}", err);
    err.chain()
        .skip(1)
        .for_each(|cause| error!("because: {}", cause));
    ErrorInternalServerError(err)
}

pub fn cache_header(max_age: usize) -> DefaultHeaders {
    DefaultHeaders::new().add(("Cache-Control", format!("public, max-age={max_age}")))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Sets up default logging if none is set for this project & actix web
    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            "actix_web=INFO,divedb=DEBUG,async_graphql=DEBUG,tantivy=DEBUG,activitypub_federation=DEBUG",
        );
    }

    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_env_var("RUST_LOG")
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().with_level(true))
        .init();

    // Grab the connect_url from the args
    let config = ConfigContext::parse();

    if config.write_schema {
        tokio::fs::write("schema.graphql", schema().sdl()).await?;
    }

    let frontend_cache = HybridCacheBuilder::new()
        .memory(1024)
        .storage(Engine::Large(LargeEngineOptions::default()))
        .with_device_options(DirectFsDeviceOptions::new(&config.cache_dir))
        .build()
        .await?;

    // Starts up the db.  This can take time to timeout if there are issues connecting
    let handle = DbHandle::new(&config.connect_url, frontend_cache.clone()).await?;
    if config.refresh_photos {
        let limit = Some(10);
        let mut offset = 0;
        loop {
            let photos = handle
                .photos(
                    None,
                    &PhotoQuery {
                        offset: Some(offset),
                        limit,
                        ..Default::default()
                    },
                )
                .await?;

            if photos.is_empty() {
                info!("Finished!");
                return Ok(());
            }

            offset += photos.len();

            let mut handles = Vec::new();
            for mut photo in photos {
                let user = handle.user_details(photo.user_id).await?;
                handles.push(tokio::task::spawn_blocking(move || {
                    debug!("Resizing:{}", photo.id);
                    resize_image(&photo, &user, true)?;
                    let (width, height) = image_dims(photo.orig_location())?;

                    photo.width = width;
                    photo.height = height;

                    Ok(photo) as Result<Photo, Error>
                }));
            }

            for join_handle in handles {
                let photo = join_handle.await??;

                handle
                    .update_dims(photo.id, photo.width, photo.height)
                    .await?;
            }
        }
    }

    let dive_batch = DiveSiteBatcher::new(&handle);
    let sealife_batch = SealifeBatcher::new(&handle);

    debug!("Warming Searcher");

    let searcher = Searcher::new();
    searcher.build_index(&handle).await?;

    let emailer = Emailer::new(&config);

    // Starts up the server with our routes
    debug!("Starting server, listening on: {}", config.listen_address);

    let site_context = config.site_context;

    let rate_limiter = get_limiter();

    let cipher =
        Aes256Gcm::new_from_slice(&site_context.get_key()).map_err(|val| anyhow!("{}", val))?;

    let site_url = Url::parse(&SITE_URL)?;

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let openid_client = if let Some(ref open_id_settings) = config.open_id_settings {
        Some(OpenIDClient::new(open_id_settings, client.clone()).await?)
    } else {
        None
    };

    let web_context = Arc::new(WebContext {
        handle,
        rate_limiter,
        cipher,
        searcher,
        emailer,
        site_context,
        dive_batch,
        sealife_batch,
        facebook: config.facebook,
        frontend_cache,
        frontend_url: config.frontend_url,
        client: client.clone(),
        admin_email: config.admin_email,
        openid_client,
    });

    let domain = site_url
        .domain()
        .ok_or_else(|| anyhow!("No domain from url: {site_url}"))?;

    let fed_config = FederationConfig::builder()
        .domain(domain)
        .app_data(web_context.clone())
        .client(client.clone().into())
        .http_signature_compat(true)
        .build()
        .await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(graphql::schema()))
            .app_data(Data::new(client.clone()))
            .app_data(Data::from(web_context.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(FederationMiddleware::new(fed_config.clone()))
            .service(robots)
            .service(sitemap_handler)
            .service(
                web::resource("/api/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/api/playground").route(web::get().to(playground_handler)))
            .service(
                web::resource("/api/chart/{dive_id}")
                    .wrap(cache_header(604800))
                    .route(web::get().to(svg_chart)),
            )
            .service(
                web::resource("/api/chart/{dive_id}/png")
                    .wrap(cache_header(604800))
                    .route(web::get().to(png_chart)),
            )
            .service(
                web::resource(["/api/photos/{kind}/{id}"])
                    .wrap(cache_header(604800))
                    .route(web::get().to(open_photo)),
            )
            .service(web::resource("/api/photos").route(web::post().to(save_files)))
            .service(activitypub::configure_users())
            .service(activitypub::configure_dives())
            .service(
                web::resource("/.well-known/webfinger")
                    .route(web::get().to(activitypub::webfinger)),
            )
            .route(
                "/nodeinfo/2.0.json",
                web::get()
                    .to(activitypub::node_info)
                    .wrap(cache_header(3600)),
            )
            .route(
                "/.well-known/nodeinfo",
                web::get()
                    .to(activitypub::node_info_well_known)
                    .wrap(cache_header(86400)),
            )
            .service(frontend::frontend())
    })
    .bind(&config.listen_address)?
    .run()
    .await?;
    Ok(())
}

pub async fn playground_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(
            "/api/graphql",
        )))
}

#[allow(clippy::too_many_arguments)]
async fn graphql(
    gql_request: GraphQLRequest,
    http_request: HttpRequest,
    conn: ConnectionInfo,
    token: Token,
    web: web::Data<WebContext>,
    schema: web::Data<Schema>,
    fed_data: activitypub_federation::config::Data<Arc<WebContext>>,
) -> GraphQLResponse {
    let user: Option<User>;

    if let Some(user_id) = token.user_id {
        user = web.handle.user_details(user_id).await.ok();
    } else {
        user = None
    }

    // This is used with the rate limiter
    let remote_ip = conn
        .realip_remote_addr()
        .and_then(|val| val.parse().ok())
        .unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));

    let schema_context = SchemaContext {
        con: ConnectionContext { user, remote_ip },
        web: web.into_inner(),
        fed: fed_data,
    };

    let mut request = gql_request.into_inner();

    request = request.data(schema_context);

    let mut response = schema.execute(request).await;

    if http_request.method() == "GET" {
        response = response.cache_control(CacheControl {
            public: true,
            max_age: 3600,
        });
    }

    response.into()
}
