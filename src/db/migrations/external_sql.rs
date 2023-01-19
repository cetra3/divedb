use anyhow::Error;
use async_trait::async_trait;

use super::Migration;
use deadpool_postgres::Pool;

pub struct ExternalMigration {
    sql_name: &'static str,
    sql_contents: &'static str,
}

impl ExternalMigration {
    pub fn new(sql_name: &'static str, sql_contents: &'static str) -> Self {
        Self {
            sql_name,
            sql_contents,
        }
    }
}

#[async_trait]
impl Migration for ExternalMigration {
    fn name(&self) -> &str {
        self.sql_name
    }

    async fn migrate(&self, pool: &Pool) -> Result<(), Error> {
        let client = pool.get().await?;

        client.batch_execute(self.sql_contents).await?;

        Ok(())
    }
}

macro_rules! external {
    ($a: expr) => {
        crate::db::migrations::external_sql::ExternalMigration::new($a, include_str!($a))
    };
}
