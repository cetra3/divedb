use anyhow::Error;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use postgres_types::ToSql;
use reqwest::Client;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::sync::mpsc::{self, error::TrySendError, Receiver, Sender};
use tokio::sync::RwLock;
use tokio::time::Duration;
use tokio_postgres::{NoTls, Row};
use uuid::Uuid;

use crate::{db::migrations::Migrator, schema::*};
use tracing::*;

mod migrations;

mod categories;
mod comment;
mod dive;
mod dive_site;
mod feedback;
mod og_reference;
mod password_reset;
mod photo;
mod region;
mod sealife;
mod user;

#[derive(Clone)]
pub struct DbHandle {
    pool: Pool,
    pub client: Client,
    site_metrics: Arc<RwLock<HashMap<Uuid, SiteMetric>>>,
    popular_sites: Arc<RwLock<Vec<DiveSite>>>,
    photos: Arc<RwLock<HashMap<Uuid, Photo>>>,
    sender: Sender<()>,
}

// Adds in all the SQL queries needed to persist Files and Parts
impl DbHandle {
    // Sets up a new connection pool at the url specified.  If this can't connect within a timeout, it will error out.
    pub async fn new(url: &str) -> Result<Self, Error> {
        debug!("Connecting to URL:{}", url);

        let pg_config = tokio_postgres::Config::from_str(url)?;

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };

        let mgr = Manager::from_config(pg_config, NoTls, mgr_config);

        let pool = Pool::builder(mgr).max_size(16).build()?;

        Migrator::new(pool.clone()).apply_migrations().await?;

        let (sender, rx) = mpsc::channel::<()>(1);

        let site_metrics = Arc::new(RwLock::new(HashMap::new()));
        let popular_sites = Arc::new(RwLock::new(Vec::new()));
        let photos = Arc::new(RwLock::new(HashMap::new()));

        throttle_update(rx, Duration::from_secs(3600));

        Ok(DbHandle {
            pool,
            client: get_client(),
            site_metrics,
            popular_sites,
            photos,
            sender,
        })
    }

    pub fn from_pool(pool: &Pool) -> Self {
        let (sender, rx) = mpsc::channel::<()>(1);

        let site_metrics = Arc::new(RwLock::new(HashMap::new()));
        let popular_sites = Arc::new(RwLock::new(Vec::new()));
        let photos = Arc::new(RwLock::new(HashMap::new()));

        throttle_update(rx, Duration::from_secs(3600));

        DbHandle {
            pool: pool.clone(),
            client: get_client(),
            site_metrics,
            popular_sites,
            photos,
            sender,
        }
    }

    pub async fn query(&self, sql: StatementBuilder<'_>) -> Result<Vec<Row>, Error> {
        let client = self.pool.get().await?;

        trace!("SQL:{}, Params:{:?}", sql.statement, sql.params);

        let result = client.query(&*sql.statement, &sql.params).await?;

        Ok(result)
    }

    async fn clear_cache(&self) {
        {
            self.popular_sites.write().await.clear();
        }
        {
            self.site_metrics.write().await.clear();
        }
        {
            self.photos.write().await.clear();
        }

        if let Err(TrySendError::Full(())) = self.sender.try_send(()) {
            debug!("Svelte update throttled");
        }

        debug!("Cleared cache");
    }
}

fn get_client() -> Client {
    reqwest::Client::builder()
        .user_agent("DiveDB")
        .build()
        .expect("Creating reqwest client")
}

pub struct StatementBuilder<'a> {
    pub params: Vec<&'a (dyn ToSql + Sync)>,
    pub statement: String,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(sql: &str) -> Self {
        Self {
            params: Vec::new(),
            statement: String::from(sql),
        }
    }

    pub fn add_param(&mut self, sql: &str, param: &'a (dyn ToSql + Sync)) {
        if self.params.is_empty() {
            self.statement.push_str(" WHERE ");
        } else {
            self.statement.push_str(" AND ");
        }

        self.add_statement(sql, param);
    }

    pub fn add_statement(&mut self, sql: &str, param: &'a (dyn ToSql + Sync)) {
        self.params.push(param);
        self.statement.push(' ');
        self.statement
            .push_str(&sql.replace("${}", &format!("${}", self.params.len())));
        self.statement.push(' ');
    }

    pub fn add_sql(&mut self, sql: &str) {
        self.statement.push(' ');
        self.statement.push_str(sql);
        self.statement.push(' ');
    }
}

// This throttles updates to svelte so it happens only once per-duration
fn throttle_update(rx: Receiver<()>, duration: Duration) {
    use tokio_stream::wrappers::ReceiverStream;
    use tokio_stream::StreamExt;

    tokio::spawn(async move {
        let throttled_stream = ReceiverStream::new(rx).throttle(duration);

        tokio::pin!(throttled_stream);
        loop {
            throttled_stream.next().await;
            if let Err(err) = tokio::fs::File::create("./front/build/notify.lock").await {
                error!("Error notifying svelte frontend: {}", err);
            }
        }
    });
}
