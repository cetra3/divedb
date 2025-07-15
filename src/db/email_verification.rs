use anyhow::Error;
use divedb_core::FromRow;
use uuid::Uuid;

use crate::schema::*;

use super::{DbHandle, StatementBuilder};

impl DbHandle {
    pub async fn request_email_verification(
        &self,
        user_id: Uuid,
    ) -> Result<EmailVerification, Error> {
        let id = Uuid::new_v4();
        let client = self.pool.get().await?;
        let query = "insert into email_verification (id, user_id) values ($1, $2) returning *";
        let result = client.query_one(query, &[&id, &user_id]).await?;

        EmailVerification::from_row(result)
    }

    pub async fn get_valid_email_verifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<EmailVerification>, Error> {
        let mut sql = StatementBuilder::new("select id, user_id, \"date\" from email_verification");

        sql.add_param("user_id = ${}", &user_id);

        sql.add_sql(" AND \"date\" > now() - interval '24 hours' ");

        EmailVerification::from_rows(self.query(sql).await?)
    }

    pub async fn mark_email_verified(&self, user_id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;

        let sql = "update users set email_verified = true where id = $1";
        client.execute(sql, &[&user_id]).await?;

        let sql = "delete from email_verification where user_id = $1";
        client.execute(sql, &[&user_id]).await?;

        Ok(())
    }
}
