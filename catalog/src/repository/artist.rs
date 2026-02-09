use async_trait::async_trait;
use sqlx::AnyPool;

use crate::{
    model::{CatalogItem, artist::Artist},
    repository::{Repository, RepositoryError},
};

const ARTIST_TABLE_NAME: &'_ str = "artists";

pub struct ArtistRepository {
    pool: AnyPool,
}

impl ArtistRepository {
    pub fn new(pool: AnyPool) -> Self {
        ArtistRepository { pool }
    }
}

#[async_trait]
impl Repository for ArtistRepository {
    type Item = CatalogItem<Artist>;

    async fn create(&mut self, item: Self::Item) -> Result<Self::Item, RepositoryError> {
        sqlx::query(&format!(
            "INSERT INTO {} (id, metadata) VALUES ($1, $2)",
            ARTIST_TABLE_NAME
        ))
        .bind(item.id)
        .bind(serde_json::to_string(&item.metadata).map_err(|_| RepositoryError::ItemCreate)?)
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::ItemCreate)?;

        Ok(item)
    }

    async fn read(&mut self, id: &i64) -> Result<Self::Item, RepositoryError> {
        let catalog_item: CatalogItem<Artist> = sqlx::query_as(&format!(
            "SELECT id, metadata FROM {} WHERE id = $1",
            ARTIST_TABLE_NAME
        ))
        .bind(&id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| RepositoryError::ItemCreate)?;

        Ok(catalog_item)
    }
}
