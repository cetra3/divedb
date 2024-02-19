use anyhow::Error;
use divedb_core::FromRow;
use uuid::Uuid;

use crate::schema::*;

use super::{DbHandle, StatementBuilder};

impl DbHandle {
    pub async fn create_comment(
        &self,
        user_id: Uuid,
        request: &CreateDiveComment,
    ) -> Result<DiveComment, Error> {
        let uuid = request.id.unwrap_or_else(Uuid::new_v4);

        let client = self.pool.get().await?;
        let query = "insert into dive_comments (id, dive_id, user_id, description, date)
            values ($1, $2, $3, $4, now())
            
            on conflict(id) do update
                set date = excluded.date,
                    description = excluded.description
            
            returning *";

        let result = client
            .query_one(
                query,
                &[&uuid, &request.dive_id, &user_id, &request.description],
            )
            .await?;

        DiveComment::from_row(result)
    }

    pub async fn new_external_comment(
        &self,
        dive_id: Uuid,
        user_id: Uuid,
        description: &str,
        ap_id: &str,
    ) -> Result<DiveComment, Error> {
        let uuid = Uuid::new_v4();
        let client = self.pool.get().await?;

        let query = "
            insert into dive_comments (
                id,
                dive_id,
                user_id,
                description,
                ap_id,
                external,
                date
            ) 
            values (
                $1,
                $2,
                $3,
                $4,
                $5,
                true,
                now()
            ) returning *";
        let result = client
            .query_one(query, &[&uuid, &dive_id, &user_id, &description, &ap_id])
            .await?;

        DiveComment::from_row(result)
    }

    pub async fn remove_comment(&self, id: Uuid, user_id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from dive_comments where id = $1 and user_id = $2";
        client.execute(query, &[&id, &user_id]).await?;

        Ok(())
    }

    pub async fn dive_comments(&self, dive_id: Uuid) -> Result<i64, Error> {
        let client = self.pool.get().await?;
        let query = "select count(*) from dive_comments where dive_id = $1";

        let result = client.query_one(query, &[&dive_id]).await?;
        let count: i64 = result.get(0);

        Ok(count)
    }

    pub async fn comment_by_ap_id(&self, ap_id: &str) -> Result<DiveComment, Error> {
        let client = self.pool.get().await?;
        let query = "select * from dive_comments where ap_id = $1";
        let result = client.query_one(query, &[&ap_id]).await?;

        DiveComment::from_row(result)
    }

    pub async fn comments(&self, query: &DiveCommentQuery) -> Result<Vec<DiveComment>, Error> {
        let mut sql = StatementBuilder::new("select * from dive_comments");

        if let Some(ref id) = query.id {
            sql.add_param("id = ${}", id);
        }

        if let Some(ref user_id) = query.user_id {
            sql.add_param("user_id = ${}", user_id);
        }

        if let Some(ref dive_id) = query.dive_id {
            sql.add_param("dive_id = ${}", dive_id);
        }

        sql.add_sql("order by \"date\" desc");

        DiveComment::from_rows(self.query(sql).await?)
    }
}
