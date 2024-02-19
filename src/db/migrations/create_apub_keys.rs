use activitypub_federation::http_signatures::generate_actor_keypair;
use anyhow::Error;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use divedb_core::FromRow;
use uuid::Uuid;

use super::Migration;

pub struct CreateApubKeys;

#[async_trait]
impl Migration for CreateApubKeys {
    fn name(&self) -> &str {
        "create_apub_keys"
    }

    async fn migrate(&self, pool: &Pool) -> Result<(), Error> {
        let mut client = pool.get().await?;

        client
            .batch_execute(
                "
                alter table users add column public_key text;
                alter table users add column private_key text;
            ",
            )
            .await?;

        let result = client.query("select id from users", &[]).await?;

        let users = User::from_rows(result)?;

        let transaction = client.transaction().await?;
        let statement = transaction
            .prepare("update users set public_key = $1, private_key = $2 where id = $3")
            .await?;

        for user in users {
            let key_pair = generate_actor_keypair()?;

            transaction
                .execute(
                    &statement,
                    &[&key_pair.public_key, &key_pair.private_key, &user.id],
                )
                .await?;
        }

        transaction.commit().await?;

        client
            .batch_execute(
                "
                alter table users alter column public_key set not null;
            ",
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
}
