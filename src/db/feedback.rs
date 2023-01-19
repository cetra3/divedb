use anyhow::Error;
use divedb_core::FromRow;
use uuid::Uuid;

use crate::schema::*;

use super::DbHandle;

impl DbHandle {
    pub async fn add_feedback(&self, user_id: Uuid, feedback: String) -> Result<Feedback, Error> {
        let id = Uuid::new_v4();
        let client = self.pool.get().await?;
        let query = "insert into feedback (id, user_id, feedback) values ($1, $2, $3) returning *";
        let result = client.query_one(query, &[&id, &user_id, &feedback]).await?;

        Ok(Feedback::from_row(result)?)
    }
}
