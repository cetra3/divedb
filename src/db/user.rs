use anyhow::Error;
use divedb_core::FromRow;
use uuid::Uuid;

use crate::schema::*;

use super::DbHandle;

impl DbHandle {
    pub async fn new_user(&self, email: &str, hash: &str) -> Result<User, Error> {
        let uuid = Uuid::new_v4();
        let client = self.pool.get().await?;
        let query =  "insert into users (id, email, password, level) values ($1, $2, $3, 'User') returning *";
        let result = client.query_one(query, &[&uuid, &email, &hash]).await?;

        Ok(User::from_row(result)?)
    }

    pub async fn user(&self, email: &str) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "select * from users where lower(email) = lower($1)";
        let result = client.query_one(query, &[&email]).await?;

        Ok(User::from_row(result)?)
    }

    pub async fn user_details(&self, id: Uuid) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "select * from users where id = $1";
        let result = client.query_one(query, &[&id]).await?;

        Ok(User::from_row(result)?)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;

        let query = "delete from users where id = $1";

        client.execute(query, &[&id]).await?;

        Ok(())
    }

    pub async fn update_password(&self, email: &str, password: &str) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "update users set password = $2 where lower(email) = lower($1) returning *";
        let result = client.query_one(query, &[&email, &password]).await?;

        Ok(User::from_row(result)?)
    }

    pub async fn update_settings(
        &self,
        email: &str,
        username: Option<String>,
        watermark_location: OverlayLocation,
        copyright_location: Option<OverlayLocation>,
    ) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "update users set username = $1, watermark_location = $2, copyright_location = $3 where lower(email) = lower($4) returning *";
        let result = client
            .query_one(
                query,
                &[&username, &watermark_location, &copyright_location, &email],
            )
            .await?;

        Ok(User::from_row(result)?)
    }

    pub async fn photo_quota_usage(&self, user_id: Uuid) -> Result<i64, Error> {
        let client = self.pool.get().await?;

        let query = client
            .query(
                "select sum(size) from photos where user_id = $1 ",
                &[&user_id],
            )
            .await?;

        Ok(query.get(0).and_then(|row| row.get(0)).unwrap_or(0))
    }
}
