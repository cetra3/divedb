use anyhow::Error;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use divedb_core::FromRow;

use crate::{db::DbHandle, schema::Photo};

use log::*;

use super::Migration;

pub struct FixPhotoDiveIds;

#[async_trait]
impl Migration for FixPhotoDiveIds {
    fn name(&self) -> &str {
        "fix_photo_dive_ids"
    }

    async fn migrate(&self, pool: &Pool) -> Result<(), Error> {
        let mut client = pool.get().await?;

        let sql = "select 
                id,
                user_id,
                filename,
                date,
                dive_id,
                dive_site_id,
                size,
                upload_date,
                location.hide_location
            from 
                photos as p
            left join lateral 
                (select 
                    hide_location
                 from sealife_tags as st
                 inner join sealife s on s.id = st.sealife_id
                 where p.id = st.photo_id
                ) location on true";

        let photos = Photo::from_rows(client.query(sql, &[]).await?)?;

        let conn = client.transaction().await?;
        let statement = conn
            .prepare("update photos set dive_id = $1 where id = $2")
            .await?;

        let handle = DbHandle::from_pool(pool);

        for photo in photos {
            if photo.dive_id.is_none() {
                if let Some(date) = photo.date {
                    if let Some(dive) = handle.nearest_dive_by_time(photo.user_id, date).await? {
                        debug!("Setting dive id to {} for photo {}", dive.id, photo.id);
                        conn.execute(&statement, &[&dive.id, &photo.id]).await?;
                    }
                }
            }
        }

        conn.commit().await?;

        Ok(())
    }
}
