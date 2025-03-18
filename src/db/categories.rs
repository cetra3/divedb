use std::collections::HashMap;

use anyhow::Error;
use divedb_core::FromRow;
use postgres_types::ToSql;
use tracing::*;
use uuid::Uuid;

use crate::schema::*;

use super::{DbHandle, StatementBuilder};

impl DbHandle {
    pub async fn create_category(&self, request: CreateCategory) -> Result<Category, Error> {
        let uuid = request.id.unwrap_or_else(Uuid::new_v4);

        let client = self.pool.get().await?;
        let query = "insert into categories (id, name)
            values ($1, $2)
            
            on conflict(id) do update
                set name = excluded.name
            
            returning *";

        let params: &[&(dyn ToSql + Sync)] = &[&uuid, &request.name];

        let result = client.query_one(query, params).await?;

        Category::from_row(result)
    }

    pub async fn create_category_value(
        &self,
        request: CreateCategoryValue,
    ) -> Result<CategoryValue, Error> {
        let uuid = request.id.unwrap_or_else(Uuid::new_v4);

        let client = self.pool.get().await?;
        let query = "insert into category_values (id, category_id, \"value\")
            values ($1, $2, $3)
            
            on conflict(id) do update
                set category_id = excluded.category_id,
                value = excluded.value
            
            returning *";

        let params: &[&(dyn ToSql + Sync)] = &[&uuid, &request.category_id, &request.value];

        let result = client.query_one(query, params).await?;

        CategoryValue::from_row(result)
    }

    pub async fn categories(&self) -> Result<Vec<Category>, Error> {
        let client = self.pool.get().await?;
        let query = "select * from categories";

        let result = client.query(query, &[]).await?;

        Category::from_rows(result)
    }

    pub async fn category_values(
        &self,
        category_id: Option<Uuid>,
    ) -> Result<Vec<CategoryValue>, Error> {
        let mut sql = StatementBuilder::new("select * from category_values");

        if let Some(ref id) = category_id {
            sql.add_param("category_id = ${}", id);
        }

        let result = self.query(sql).await?;

        CategoryValue::from_rows(result)
    }

    pub async fn category_map(&self, sealife_id: Uuid) -> Result<CategoryMap, Error> {
        let mut sql = StatementBuilder::new("select category_id, category_value_id  from sealife_category_values inner join category_values on category_values.id = category_value_id");

        sql.add_param("sealife_id = ${}", &sealife_id);

        let result = self.query(sql).await?;

        let mut category_map: CategoryMap = HashMap::new();

        for row in result {
            let category_id: Uuid = row.try_get(0)?;
            let category_value_id: Uuid = row.try_get(1)?;

            if let Some(vec) = category_map.get_mut(&category_id) {
                vec.push(category_value_id);
            } else {
                category_map.insert(category_id, vec![category_value_id]);
            }
        }

        Ok(category_map)
    }

    pub async fn create_category_map(
        &self,
        sealife_id: Uuid,
        category_map: CategoryMap,
    ) -> Result<(), Error> {
        let query = "insert into sealife_category_values (id, sealife_id, category_value_id) values ($1, $2, $3)";

        let mut client = self.pool.get().await?;

        let conn = client.transaction().await?;

        conn.query(
            "delete from sealife_category_values where sealife_id = $1",
            &[&sealife_id],
        )
        .await?;

        let statement = conn.prepare(query).await?;

        for value_id in category_map.values().flatten() {
            debug!("Value Id:{}", value_id);
            conn.execute(&statement, &[&Uuid::new_v4(), &sealife_id, &value_id])
                .await?;
        }

        conn.commit().await?;

        Ok(())
    }

    pub async fn remove_category(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from categories where id = $1";

        client.execute(query, &[&id]).await?;
        Ok(())
    }

    pub async fn remove_category_value(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from category_value where id = $1";

        client.execute(query, &[&id]).await?;

        Ok(())
    }
}
