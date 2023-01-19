use std::collections::HashMap;

use anyhow::Error;
use divedb_core::FromRow;
use uuid::Uuid;

use crate::schema::*;

use super::{DbHandle, StatementBuilder};

impl DbHandle {
    pub async fn create_sealife(&self, sealife: &CreateSealife) -> Result<Sealife, Error> {
        let client = self.pool.get().await?;

        if let Some(existing_id) = sealife.id {
            let revision_query = "insert into sealife_revisions (sealife_id, name, scientific_name, description, photo_id, \"date\", slug)
            select id as sealife_id, name, scientific_name, description, photo_id, \"date\", slug from sealife where id = $1";
            client.execute(revision_query, &[&existing_id]).await?;
        }

        let uuid = sealife.id.unwrap_or_else(Uuid::new_v4);

        let query =
            "insert into sealife (id, name, scientific_name, description, photo_id, \"date\", slug, hide_location)
            values ($1, $2, $3, $4, $5, now(), slugify($2), $6)
            
            on conflict(id) do update
                set name = excluded.name,
                    scientific_name = excluded.scientific_name,
                    description = excluded.description,
                    photo_id = excluded.photo_id,
                    \"date\" = excluded.date,
                    slug = excluded.slug,
                    hide_location = excluded.hide_location
            
            returning * ";

        let result = client
            .query_one(
                query,
                &[
                    &uuid,
                    &sealife.name,
                    &sealife.scientific_name,
                    &sealife.description,
                    &sealife.photo_id,
                    &sealife.hide_location,
                ],
            )
            .await?;

        if let Some(photo_id) = sealife.photo_id {
            client
                .execute(
                    "insert into sealife_tags
                (sealife_id, photo_id, u_min, v_min, u_max, v_max)
                values ($1, $2, 0, 0, 0, 0)
                on conflict do nothing
            ",
                    &[&uuid, &photo_id],
                )
                .await?;
        }

        if let Some(ref category_map) = sealife.category_map {
            let mut uuid_map: CategoryMap = HashMap::new();

            for (key, val) in category_map {
                if let Ok(uuid_key) = key.parse() {
                    uuid_map.insert(uuid_key, val.clone());
                }
            }

            self.create_category_map(uuid, uuid_map).await?;
        }

        self.clear_cache().await;

        Ok(Sealife::from_row(result)?)
    }

    pub async fn sealife_batch(&self, uuids: &[Uuid]) -> Result<Vec<Sealife>, Error> {
        let client = self.pool.get().await?;
        let query = "select * from sealife where id = ANY($1)";

        let result = client.query(query, &[&uuids]).await?;

        Ok(Sealife::from_rows(result)?)
    }

    pub async fn sealife(&self, query: &SealifeQuery) -> Result<Vec<Sealife>, Error> {
        let mut sql = StatementBuilder::new("select * from sealife");

        if let Some(ref id) = query.id {
            sql.add_param("id = ${}", id);
        }

        if let Some(ref name) = query.name {
            sql.add_param(
                "(name ilike '%' || ${} || '%' OR scientific_name ilike '%' || ${} || '%')",
                name,
            );
        }

        if let Some(ref scientific_name) = query.scientific_name {
            sql.add_param("scientific_name ilike '%' || ${} || '%'", scientific_name);
        }

        if let Some(ref slug) = query.slug {
            sql.add_param("slug = ${}", slug);
        }

        if let Some(ref id) = query.photo_id {
            sql.add_param(
                "id in (select sealife_id from sealife_tags where photo_id = ${})",
                id,
            );
        }

        if let Some(ref ids) = query.category_values {
            for id in ids {
                sql.add_param(
                    "id in (select sealife_id from sealife_category_values where category_value_id = ${})",
                    id,
                );
            }
        }

        sql.add_sql(" order by \"date\" desc");

        Ok(Sealife::from_rows(self.query(sql).await?)?)
    }

    pub async fn remove_sealife(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from sealife where id = $1";
        client.execute(query, &[&id]).await?;

        self.clear_cache().await;

        Ok(())
    }
}
