use anyhow::Error;
use divedb_core::FromRow;
use postgres_types::ToSql;
use uuid::Uuid;

use crate::schema::*;
use log::*;

use super::{DbHandle, StatementBuilder};

impl DbHandle {
    pub async fn create_dive_site(
        &self,
        user_id: Uuid,
        request: &CreateDiveSite,
    ) -> Result<DiveSite, Error> {
        let uuid = request.id.unwrap_or_else(Uuid::new_v4);
        let client = self.pool.get().await?;

        if let Some(existing_id) = request.id {
            let revision_query = "insert into dive_sites_revision (dive_id, user_id, name, description, access, difficulty, depth, lat, lon, published)
            select id as dive_id, user_id, name, description, access, difficulty, depth, lat, lon, published from dive_sites where id = $1";
            client.execute(revision_query, &[&existing_id]).await?;
        }

        let query =
            "insert into dive_sites (id, user_id, name, description, access, difficulty, depth, lat, lon, published, photo_id, \"date\", slug)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now(), slugify($3))
            
            on conflict(id) do update
                set name = excluded.name,
                    description = excluded.description,
                    access = excluded.access,
                    difficulty = excluded.difficulty,
                    depth = excluded.depth,
                    lat = excluded.lat,
                    lon = excluded.lon,
                    published = excluded.published,
                    photo_id = excluded.photo_id,
                    \"date\" = excluded.date,
                    slug = excluded.slug
            
            returning id, user_id, name, description, access, difficulty, depth, lat, lon, published, photo_id, \"date\", slug";

        let params: &[&(dyn ToSql + Sync)] = &[
            &uuid,
            &user_id,
            &request.name,
            &request.description,
            &request.access,
            &request.difficulty,
            &request.depth,
            &request.lat,
            &request.lon,
            &request.published,
            &request.photo_id,
        ];

        let result = client.query_one(query, params).await?;

        self.clear_cache().await;

        Ok(DiveSite::from_row(result)?)
    }

    pub async fn site_metrics(&self, dive_site_id: Uuid) -> Result<Option<SiteMetric>, Error> {
        {
            let metrics = self.site_metrics.read().await;

            if metrics.len() > 0 {
                return Ok(metrics.get(&dive_site_id).cloned());
            }
        }

        let client = self.pool.get().await?;

        let mut metrics = self.site_metrics.write().await;

        if metrics.len() > 0 {
            return Ok(metrics.get(&dive_site_id).cloned());
        }

        let photo_count_query =
            "select count(*) as photo_count,dive_site_id from photos where dive_site_id is not null group by dive_site_id";

        let result = client.query(photo_count_query, &[]).await?;

        for row in result {
            let photo_count: i64 = row.try_get(0)?;
            let site_id: Uuid = row.try_get(1)?;

            if let Some(val) = metrics.get_mut(&site_id) {
                val.photo_count = photo_count as i32;
            } else {
                metrics.insert(
                    site_id,
                    SiteMetric {
                        photo_count: photo_count as i32,
                        dive_count: 0,
                    },
                );
            }
        }

        let dive_count_query =
            "select count(*) as dive_count,dive_site_id from dives where dive_site_id is not null group by dive_site_id";

        let result = client.query(dive_count_query, &[]).await?;

        for row in result {
            let dive_count: i64 = row.try_get(0)?;
            let site_id: Uuid = row.try_get(1)?;

            if let Some(val) = metrics.get_mut(&site_id) {
                val.dive_count = dive_count as i32;
            } else {
                metrics.insert(
                    site_id,
                    SiteMetric {
                        dive_count: dive_count as i32,
                        photo_count: 0,
                    },
                );
            }
        }

        debug!("Metrics:{:?}", metrics.values());

        return Ok(metrics.get(&dive_site_id).cloned());
    }

    pub async fn dive_sites_batch(&self, uuids: &[Uuid]) -> Result<Vec<DiveSite>, Error> {
        let client = self.pool.get().await?;
        let query = "select id, user_id, name, description, access, difficulty, depth, lat, lon, published, photo_id, \"date\", slug from dive_sites where id = ANY($1)";

        let result = client.query(query, &[&uuids]).await?;

        Ok(DiveSite::from_rows(result)?)
    }

    pub async fn popular_dive_sites(&self) -> Result<Vec<DiveSite>, Error> {
        {
            let cache = self.popular_sites.read().await;

            debug!("Cache hit on popular dive sites");

            if !cache.is_empty() {
                return Ok(cache.clone());
            }
        }

        let client = self.pool.get().await?;

        let sql =
            "
            select id, user_id, name, description, access, difficulty, depth, lat, lon, published, photo_id, \"date\", slug
                from dive_sites
                left outer join 
              	(select
                    count(*) as dive_count,
            	    max(\"date\") as last_dive_date,
            	    dive_site_id from dives
            	 group by 
            	    dive_site_id
            	) as dive_counts 
                on dive_counts.dive_site_id = dive_sites.id
                where published = true
                order by dive_count desc nulls last
                limit 4
            ";

        let result = DiveSite::from_rows(client.query(&*sql, &[]).await?)?;

        *self.popular_sites.write().await = result.clone();

        Ok(result)
    }

    pub async fn dive_sites(
        &self,
        user_id: Option<Uuid>,
        query: &DiveSiteQuery,
    ) -> Result<Vec<DiveSite>, Error> {
        let mut sql = StatementBuilder::new("select id, user_id, name, description, access, difficulty, depth, lat, lon, published, photo_id, \"date\", slug
                from dive_sites
                left outer join 
              	(select
                    count(*),
            	    max(\"date\") as last_dive_date,
            	    dive_site_id from dives
            	 group by 
            	    dive_site_id
            	) as dive_counts 
                on dive_counts.dive_site_id = dive_sites.id");

        if let Some(ref id) = user_id {
            sql.add_param("( user_id = ${} or published = true )", id);
        } else {
            sql.add_param("( published = ${} )", &true);
        }

        if let Some(ref id) = query.id {
            sql.add_param("id = ${}", id);
        }

        if let Some(ref max_depth) = query.max_depth {
            sql.add_param("depth < ${}", max_depth);
        }

        if let Some(ref name) = query.name {
            sql.add_param("name ilike '%' || ${} || '%'", name);
        }

        if let Some(ref slug) = query.slug {
            sql.add_param("slug = ${}", slug);
        }

        sql.add_sql(" order by last_dive_date desc nulls last");

        Ok(DiveSite::from_rows(self.query(sql).await?)?)
    }

    pub async fn merge_dive_sites(&self, from_id: Uuid, to_id: Uuid) -> Result<(), Error> {
        let mut client = self.pool.get().await?;

        let conn = client.transaction().await?;

        conn.execute(
            "update dives set dive_site_id = $1 where dive_site_id = $2",
            &[&to_id, &from_id],
        )
        .await?;
        conn.execute("delete from dive_sites where id = $1", &[&from_id])
            .await?;

        conn.commit().await?;

        self.clear_cache().await;

        Ok(())
    }

    pub async fn remove_dive_site(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from dive_sites where id = $1";
        client.execute(query, &[&id]).await?;

        self.clear_cache().await;

        Ok(())
    }
}
