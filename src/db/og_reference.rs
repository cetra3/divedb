use anyhow::Error;
use divedb_core::FromRow;
use uuid::Uuid;

use crate::{db::StatementBuilder, schema::*};

use super::DbHandle;

impl DbHandle {
    pub async fn add_reference(
        &self,
        create_reference: CreateOgReference,
    ) -> Result<OgReference, Error> {
        let og_reference = OgReference::from_url(&self.client, &create_reference.url).await?;

        let client = self.pool.get().await?;
        let query = "insert into og_reference (id, url, title, image_url, description, last_fetched) values ($1, $2, $3, $4, $5, $6)";

        client
            .execute(
                query,
                &[
                    &og_reference.id,
                    &og_reference.url,
                    &og_reference.title,
                    &og_reference.image_url,
                    &og_reference.description,
                    &og_reference.last_fetched,
                ],
            )
            .await?;

        if let Some(sealife_id) = create_reference.sealife_id {
            client
                .execute(
                    "
                  insert 
                    into 
                  sealife_og_reference 
                    (sealife_id, og_reference_id)
                  values ($1, $2)",
                    &[&sealife_id, &og_reference.id],
                )
                .await?;
        }

        if let Some(dive_site_id) = create_reference.dive_site_id {
            client
                .execute(
                    "
                  insert 
                    into 
                  dive_sites_og_reference 
                    (dive_site_id, og_reference_id)
                  values ($1, $2)",
                    &[&dive_site_id, &og_reference.id],
                )
                .await?;
        }

        Ok(og_reference)
    }

    pub async fn references(&self, query: &OgReferenceQuery) -> Result<Vec<OgReference>, Error> {
        let mut sql = StatementBuilder::new(
            "
            select 
              id,
              url,
              title,
              image_url,
              description,
              last_fetched
            from 
              og_reference as og
            ",
        );

        if let Some(ref id) = query.id {
            sql.add_param("id = ${}", id);
        }

        if let Some(ref sealife_id) = query.sealife_id {
            sql.add_param(
                "id in (select og_reference_id from sealife_og_reference where sealife_id = ${})",
                sealife_id,
            );
        }

        if let Some(ref dive_site_id) = query.dive_site_id {
            sql.add_param("id in (select og_reference_id from dive_sites_og_reference where dive_site_id = ${})", dive_site_id);
        }

        Ok(OgReference::from_rows(self.query(sql).await?)?)
    }

    pub async fn remove_reference(&self, id: Uuid) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let query = "delete from og_reference where id = $1";
        client.execute(query, &[&id]).await?;

        Ok(())
    }
}
