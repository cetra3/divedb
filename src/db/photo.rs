use anyhow::{anyhow, Error};
use divedb_core::FromRow;
use log::*;
use uuid::Uuid;

use crate::schema::*;

use super::{DbHandle, StatementBuilder};

impl DbHandle {
    pub async fn update_dims(&self, id: Uuid, width: i32, height: i32) -> Result<(), Error> {
        let client = self.pool.get().await?;

        let query = "update photos set width = $1, height = $2 where id = $3";
        client.execute(query, &[&width, &height, &id]).await?;
        Ok(())
    }

    pub async fn add_photo(&self, create_photo: &CreatePhoto) -> Result<Photo, Error> {
        let client = self.pool.get().await?;
        let uuid = create_photo.id.unwrap_or_else(Uuid::new_v4);

        let query = "insert into photos (id, user_id, filename, date, dive_id, size, dive_site_id)
            values ($1, $2, $3, $4, $5, $6, $7)
            
            on conflict(id) do update
                set filename = excluded.filename,
                    date = excluded.date,
                    dive_id = excluded.dive_id,
                    size = excluded.size,
                    dive_site_id = excluded.dive_site_id
            
            returning 
                id,
                user_id,
                filename,
                date,
                dive_id,
                dive_site_id,
                size,
                upload_date,
                false as hide_location,
                width,
                height,
                description,
                copyright";

        let result = client
            .query_one(
                query,
                &[
                    &uuid,
                    &create_photo.user_id,
                    &create_photo.filename,
                    &create_photo.date,
                    &create_photo.dive_id,
                    &create_photo.size,
                    &create_photo.dive_site_id,
                ],
            )
            .await?;

        self.clear_cache().await;

        let photo = Photo::from_row(result)?;

        client
            .execute("delete from sealife_tags where photo_id = $1", &[&photo.id])
            .await?;

        if let Some(sealife_id) = create_photo.sealife_id {
            client
                .execute(
                    "insert into sealife_tags
                (sealife_id, photo_id, u_min, v_min, u_max, v_max)
                values ($1, $2, 0, 0, 0, 0)
            ",
                    &[&sealife_id, &photo.id],
                )
                .await?;
        }

        Ok(photo)
    }

    pub async fn photos(
        &self,
        user: Option<&User>,
        query: &PhotoQuery,
    ) -> Result<Vec<Photo>, Error> {
        if let Some(ref id) = query.id {
            // Check the cache first
            let cache = self.photos.read().await;

            if let Some(val) = cache.get(id) {
                debug!("Cache hit on photo: {}", id);

                return Ok(vec![val.clone()]);
            }
        }

        let mut sql = StatementBuilder::new(
            "
            select 
                id,
                user_id,
                filename,
                date,
                dive_id,
                dive_site_id,
                size,
                upload_date,
                location.hide_location,
                width,
                height,
                description,
                copyright
            from 
                photos as p
            left join lateral 
                (select 
                    hide_location
                 from sealife_tags as st
                 inner join sealife s on s.id = st.sealife_id
                 where p.id = st.photo_id
                ) location on true",
        );

        if let Some(ref id) = query.id {
            // We're expecting only one back from this
            sql.add_param("id = ${}", id);

            let photo = Photo::from_rows(self.query(sql).await?)?
                .pop()
                .ok_or_else(|| anyhow!("No photo found"))?;

            self.photos.write().await.insert(*id, photo.clone());

            return Ok(vec![photo]);
        }

        if let Some(ref user_id) = query.user_id {
            sql.add_param("user_id = ${}", user_id);
        }

        if let Some(ref dive_site) = query.dive_site {
            sql.add_param("dive_site_id = ${}", dive_site);

            if let Some(user) = user {
                if !user.is_editor() {
                    sql.add_param(
                        "(location.hide_location is not true or user_id = ${})",
                        &user.id,
                    );
                }
            } else {
                sql.add_param(
                    "(location.hide_location is null or location.hide_location != ${})",
                    &true,
                );
            }
        }

        if let Some(ref dive) = query.dive {
            sql.add_param("dive_id = ${}", dive);
        }

        if let Some(ref id) = query.sealife_id {
            sql.add_param(
                "id in (select photo_id from sealife_tags where sealife_id = ${})",
                id,
            );
        }

        if query.order_by_upload == Some(true) {
            sql.add_sql(" order by \"upload_date\" desc");
        } else {
            sql.add_sql(" order by \"date\" desc");
        }

        let limit = query.limit.unwrap_or(10) as i64;
        let offset = query.offset.unwrap_or(0) as i64;

        sql.add_statement("limit ${}", &limit);
        sql.add_statement("offset ${}", &offset);

        Photo::from_rows(self.query(sql).await?)
    }

    pub async fn photo_filename(&self, id: Uuid) -> Result<String, Error> {
        let client = self.pool.get().await?;

        let query = client
            .query_one("select filename from photos where id = $1", &[&id])
            .await?;

        Ok(query.try_get(0)?)
    }

    pub async fn remove_photo(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from photos where id = $1";
        client.execute(query, &[&id]).await?;

        self.clear_cache().await;

        Ok(())
    }

    pub async fn photo_likes(&self, photo_id: Uuid) -> Result<i64, Error> {
        let client = self.pool.get().await?;
        let query = "select count(*) from photo_likes where photo_id = $1";

        let result = client.query_one(query, &[&photo_id]).await?;
        let count: i64 = result.get(0);

        Ok(count)
    }

    pub async fn photo_liked(&self, user_id: Uuid, photo_id: Uuid) -> Result<bool, Error> {
        let client = self.pool.get().await?;
        let query = "select count(*) from photo_likes where user_id = $1 and photo_id = $2";

        let result = client.query_one(query, &[&user_id, &photo_id]).await?;
        let count: i64 = result.get(0);

        Ok(count > 0)
    }

    pub async fn like_photo(&self, user_id: Uuid, photo_id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query =
            "insert into photo_likes (photo_id, user_id) values ($1,$2) on conflict do nothing";
        client.execute(query, &[&photo_id, &user_id]).await?;
        Ok(())
    }

    pub async fn unlike_photo(&self, user_id: Uuid, photo_id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from photo_likes where photo_id = $1 and user_id = $2";
        client.execute(query, &[&photo_id, &user_id]).await?;
        Ok(())
    }
}
