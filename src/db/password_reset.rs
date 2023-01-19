use anyhow::Error;
use divedb_core::FromRow;
use uuid::Uuid;

use crate::schema::*;

use super::{DbHandle, StatementBuilder};

impl DbHandle {
    pub async fn request_reset(&self, user_id: Uuid) -> Result<PasswordReset, Error> {
        let id = Uuid::new_v4();
        let client = self.pool.get().await?;
        let query = "insert into password_reset (id, user_id) values ($1, $2) returning *";
        let result = client.query_one(query, &[&id, &user_id]).await?;

        Ok(PasswordReset::from_row(result)?)
    }

    pub async fn get_valid_resets(&self, user_id: Uuid) -> Result<Vec<PasswordReset>, Error> {
        let mut sql = StatementBuilder::new("select id, user_id, \"date\" from password_reset");

        sql.add_param("user_id = ${}", &user_id);

        sql.add_sql(" AND \"date\" > now() - interval '24 hours' ");

        Ok(PasswordReset::from_rows(self.query(sql).await?)?)
    }

    pub async fn remove_reset(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;

        let sql =
            "delete from password_reset where \"date\" > now() - interval '24 hours' OR id = $1";

        client.execute(sql, &[&id]).await?;

        Ok(())
    }
}
