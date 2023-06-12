use anyhow::Error;
use divedb_core::FromRow;
use uuid::Uuid;

use crate::{db::StatementBuilder, schema::*};

use super::DbHandle;

impl DbHandle {
    pub async fn add_region(&self, create_region: CreateRegion) -> Result<Region, Error> {
        let client = self.pool.get().await?;
        let uuid = create_region.id.unwrap_or_else(Uuid::new_v4);

        let query = "insert into regions (id, name, lat_min, lon_min, lat_max, lon_max, slug)
            values ($1, $2, $3, $4, $5, $6, slugify($2))
            
            on conflict(id) do update
                set name = excluded.name,
                    lat_min = excluded.lat_min,
                    lon_min = excluded.lon_min,
                    lat_max = excluded.lat_max,
                    lon_max = excluded.lon_max,
                    slug = excluded.slug

            returning *
        ";

        let result = client
            .query_one(
                query,
                &[
                    &uuid,
                    &create_region.name,
                    &create_region.lat_min,
                    &create_region.lon_min,
                    &create_region.lat_max,
                    &create_region.lon_max,
                ],
            )
            .await?;

        Region::from_row(result)
    }

    pub async fn regions(&self) -> Result<Vec<Region>, Error> {
        let sql = StatementBuilder::new(
            "
            select 
              id, name, lat_min, lon_min, lat_max, lon_max, slug
            from 
              regions order by name asc
            ",
        );

        Region::from_rows(self.query(sql).await?)
    }

    pub async fn remove_region(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from regions where id = $1";
        client.execute(query, &[&id]).await?;

        Ok(())
    }
}
