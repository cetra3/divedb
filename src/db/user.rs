use activitypub_federation::http_signatures::generate_actor_keypair;
use anyhow::Error;
use divedb_core::FromRow;
use url::Url;
use uuid::Uuid;

use crate::schema::*;

use super::DbHandle;

impl DbHandle {
    pub async fn new_user(
        &self,
        username: &str,
        email: &str,
        hash: Option<&str>,
    ) -> Result<User, Error> {
        let uuid = Uuid::new_v4();
        let client = self.pool.get().await?;

        let key_pair = generate_actor_keypair()?;

        let query =  "insert into users (id, username, email, password, level, public_key, private_key) values ($1, slugify($2), lower($3), $4, 'User', $5, $6) returning *";
        let result = client
            .query_one(
                query,
                &[
                    &uuid,
                    &username,
                    &email,
                    &hash,
                    &key_pair.public_key,
                    &key_pair.private_key,
                ],
            )
            .await?;

        User::from_row(result)
    }

    pub async fn new_external_user(
        &self,
        username: &str,
        ap_id: &Url,
        inbox: &str,
        public_key: &str,
    ) -> Result<User, Error> {
        let uuid = Uuid::new_v4();
        let client = self.pool.get().await?;

        let username = format!("{}@{}", &username, ap_id.domain().unwrap_or_default());

        let query = "
            insert into users (
                id,
                username,
                ap_id,
                inbox,
                public_key,
                level,
                external
            )
            values (
                $1,
                $2,
                $3,
                $4,
                $5,
                'User',
                true
            ) returning *";
        let result = client
            .query_one(
                query,
                &[&uuid, &username, &ap_id.as_str(), &inbox, &public_key],
            )
            .await?;

        User::from_row(result)
    }

    pub async fn followers(&self, user_id: Uuid) -> Result<Vec<User>, Error> {
        let client = self.pool.get().await?;
        let query = "select * from users where id = ANY(select followed_by_id from followers where user_id = $1)";

        let result = client.query(query, &[&user_id]).await?;

        User::from_rows(result)
    }

    pub async fn new_follow(&self, user_id: Uuid, followed_by_id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;

        let query = "insert into followers (user_id, followed_by_id) values ($1, $2) on conflict do nothing";

        client.execute(query, &[&user_id, &followed_by_id]).await?;

        Ok(())
    }

    pub async fn remove_follow(&self, user_id: Uuid, followed_by_id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;

        let query = "delete from followers where user_id = $1 and followed_by_id = $2";

        client.execute(query, &[&user_id, &followed_by_id]).await?;

        Ok(())
    }

    pub async fn user(&self, email: &str) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "select * from users where lower(email) = lower($1)";
        let result = client.query_one(query, &[&email]).await?;

        User::from_row(result)
    }

    pub async fn user_by_username(&self, username: &str) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "select * from users where lower(username) = lower($1) and external = false";
        let result = client.query_one(query, &[&username]).await?;

        User::from_row(result)
    }

    pub async fn user_by_ap_id(&self, ap_id: &str) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "select * from users where ap_id = $1";
        let result = client.query_one(query, &[&ap_id]).await?;

        User::from_row(result)
    }

    pub async fn user_details(&self, id: Uuid) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "select * from users where id = $1";
        let result = client.query_one(query, &[&id]).await?;

        User::from_row(result)
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

        User::from_row(result)
    }

    pub async fn update_settings(
        &self,
        email: &str,
        display_name: Option<String>,
        watermark_location: OverlayLocation,
        copyright_location: Option<OverlayLocation>,
        description: String,
        photo_id: Option<Uuid>,
    ) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let query = "update users set display_name = $1, watermark_location = $2, copyright_location = $3, description = $4, photo_id = $5 where lower(email) = lower($6) returning *";
        let result = client
            .query_one(
                query,
                &[
                    &display_name,
                    &watermark_location,
                    &copyright_location,
                    &description,
                    &photo_id,
                    &email,
                ],
            )
            .await?;

        User::from_row(result)
    }

    pub async fn photo_quota_usage(&self, user_id: Uuid) -> Result<i64, Error> {
        let client = self.pool.get().await?;

        let query = client
            .query(
                "select sum(size) from photos where user_id = $1 ",
                &[&user_id],
            )
            .await?;

        Ok(query.first().and_then(|row| row.get(0)).unwrap_or(0))
    }
}
