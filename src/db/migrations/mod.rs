use std::collections::HashSet;

use anyhow::Error;
use async_trait::async_trait;

use log::*;

#[macro_use]
mod external_sql;

mod fix_photo_dive_ids;

use deadpool_postgres::Pool;
use divedb_core::FromRow;

pub struct Migrator {
    pool: Pool,
    migrations: Vec<Box<dyn Migration>>,
}

#[async_trait]
pub trait Migration {
    fn name(&self) -> &str;

    async fn migrate(&self, pool: &Pool) -> Result<(), Error>;
}

impl Migrator {
    pub fn new(pool: Pool) -> Self {
        Migrator {
            pool,
            migrations: vec![
                Box::new(external!("V001__initial_setup.sql")),
                Box::new(external!("V002__regions_dive_site_photo.sql")),
                Box::new(external!("V003__add_date_field.sql")),
                Box::new(external!("V004__slugify_sites.sql")),
                Box::new(external!("V005__photo_site.sql")),
                Box::new(external!("V006__sealife.sql")),
                Box::new(external!("V007__user_name.sql")),
                Box::new(external!("V008__populate_categories.sql")),
                Box::new(external!("V009__fix_categories.sql")),
                Box::new(external!("V010__photo_upload_date.sql")),
                Box::new(external!("V011__sealife_revisions.sql")),
                Box::new(external!("V012__feedback_table.sql")),
                Box::new(external!("V013__watermark_features.sql")),
                Box::new(external!("V014__sealife_hide_location.sql")),
                Box::new(external!("V015__open_graph_references.sql")),
                Box::new(external!("V016__passwords.sql")),
                Box::new(external!("V017__photo_fields.sql")),
                Box::new(fix_photo_dive_ids::FixPhotoDiveIds),
                Box::new(external!("V018__regions_slug.sql")),
                Box::new(external!("V019__social.sql")),
            ],
        }
    }

    pub async fn apply_migrations(&self) -> Result<(), Error> {
        // Add in a check to make sure the table is created
        let client = self.pool.get().await?;

        client
            .execute(
                "
                CREATE table if not exists migrations 
                    (id serial primary key, 
                     name text not null,
                     date timestamp with time zone not null default now()
                    )",
                &[],
            )
            .await?;

        let applied =
            AppliedMigration::from_rows(client.query("select name from migrations", &[]).await?)?;

        let applied_name_set = applied
            .iter()
            .map(|val| &*val.name)
            .collect::<HashSet<&str>>();

        for migration in self.migrations.iter() {
            let migration_name = migration.name();
            if !applied_name_set.contains(migration_name) {
                info!("Applying Migration: {}", migration_name);
                migration.migrate(&self.pool).await?;

                client
                    .execute(
                        "insert into migrations (name, date) values ($1, now())",
                        &[&migration.name()],
                    )
                    .await?;
            } else {
                info!("Already applied migration: {}", migration.name());
            }
        }

        Ok(())
    }
}

#[derive(Debug, FromRow)]
struct AppliedMigration {
    name: String,
}
