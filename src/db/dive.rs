use anyhow::Error;
use chrono::{DateTime, Local};
use divedb_core::FromRow;
use uuid::Uuid;

use crate::schema::*;

use super::{DbHandle, StatementBuilder};

impl DbHandle {
    pub async fn create_dive(&self, user_id: Uuid, request: &CreateDive) -> Result<Dive, Error> {
        let uuid = request.id.unwrap_or_else(Uuid::new_v4);

        let client = self.pool.get().await?;
        let query = "insert into dives (id, user_id, date, duration, depth, dive_site_id)
            values ($1, $2, $3, $4, $5, $6)
            
            on conflict(id) do update
                set date = excluded.date,
                    dive_site_id = excluded.dive_site_id,
                    duration = excluded.duration,
                    depth = excluded.depth
            
            returning *";

        let depth = request.depth as f32;

        let result = client
            .query_one(
                query,
                &[
                    &uuid,
                    &user_id,
                    &request.date,
                    &request.duration,
                    &depth,
                    &request.dive_site_id,
                ],
            )
            .await?;

        //update photos
        let photo_query = "update photos set dive_id = $1 where dive_id is null and user_id = $2 and ABS(extract(epoch from (date - $3))) < ($4 + 7200)";

        client
            .execute(
                photo_query,
                &[&uuid, &user_id, &request.date, &request.duration],
            )
            .await?;

        self.clear_cache().await;

        Ok(Dive::from_row(result)?)
    }

    pub async fn dives(&self, query: &DiveQuery) -> Result<Vec<Dive>, Error> {
        let mut sql = StatementBuilder::new("select * from dives");

        if let Some(ref id) = query.id {
            sql.add_param("id = ${}", id);
        }

        if let Some(ref user_id) = query.user_id {
            sql.add_param("user_id = ${}", user_id);
        }

        if let Some(ref max_depth) = query.max_depth {
            sql.add_param("depth < ${}", max_depth);
        }

        sql.add_sql("order by \"date\" desc");

        Ok(Dive::from_rows(self.query(sql).await?)?)
    }

    pub async fn nearest_dive_by_time(
        &self,
        user_id: Uuid,
        time: DateTime<Local>,
    ) -> Result<Option<Dive>, Error> {
        let client = self.pool.get().await?;
        let query = "
            select 
                *
            from
                (
                select 
                    *,
                    ABS(extract(epoch from (date - $2))) as abs_time
                from
                    dives 
                where
                    user_id = $1
                ) as abs
            where abs_time < (duration + 7200)
            order by abs_time
            limit 1";

        Ok(client
            .query_opt(query, &[&user_id, &time])
            .await?
            .and_then(|row| Dive::from_row(row).ok()))
    }

    pub async fn add_metrics(&self, dive_id: Uuid, request: &[DiveMetric]) -> Result<(), Error> {
        let query = "insert into dive_metrics (dive_id, time, depth, pressure, temperature) values ($1, $2, $3, $4, $5)";

        let mut client = self.pool.get().await?;

        let conn = client.transaction().await?;

        conn.query("delete from dive_metrics where dive_id = $1", &[&dive_id])
            .await?;

        let statement = conn.prepare(query).await?;

        for metric in request {
            conn.execute(
                &statement,
                &[
                    &dive_id,
                    &metric.time,
                    &metric.depth,
                    &metric.pressure,
                    &metric.temperature,
                ],
            )
            .await?;
        }

        conn.commit().await?;

        Ok(())
    }

    pub async fn dive_number(&self, dive_id: Uuid, user_id: Uuid) -> Result<i64, Error> {
        let client = self.pool.get().await?;
        let query =
            "select count(*) from dives where date <= (select date from dives where id = $1) and user_id = $2";
        let result = client.query_one(query, &[&dive_id, &user_id]).await?;

        let count: i64 = result.get(0);

        Ok(count)
    }

    pub async fn dive_metrics(&self, dive_id: Uuid) -> Result<Vec<DiveMetric>, Error> {
        let client = self.pool.get().await?;
        let query =
            "select time, depth, pressure, temperature from dive_metrics where dive_id = $1 order by time asc";
        let result = client.query(query, &[&dive_id]).await?;

        Ok(DiveMetric::from_rows(result)?)
    }

    pub async fn has_metrics(&self, dive_id: Uuid) -> Result<bool, Error> {
        let client = self.pool.get().await?;
        let query = "select count(*) from dive_metrics where dive_id = $1";
        let result = client.query_one(query, &[&dive_id]).await?;

        let count: i64 = result.get(0);

        Ok(count > 1)
    }

    pub async fn remove_dive(&self, user_id: Uuid, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from dives where id = $1 and user_id = $2";
        client.execute(query, &[&id, &user_id]).await?;
        self.clear_cache().await;

        Ok(())
    }
}
