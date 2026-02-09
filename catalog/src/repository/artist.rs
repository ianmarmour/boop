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
    type Item = Artist;

    async fn create(
        &mut self,
        item: Self::Item,
    ) -> Result<CatalogItem<Self::Item>, RepositoryError> {
        let catalog_item: CatalogItem<Artist> = sqlx::query_as(&format!(
            "INSERT INTO {} (metadata) VALUES ($1) RETURNING id, metadata",
            ARTIST_TABLE_NAME
        ))
        .bind(serde_json::to_string(&item).map_err(|_| RepositoryError::ItemCreate)?)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| RepositoryError::ItemCreate)?;

        Ok(catalog_item)
    }

    async fn read(&mut self, id: &i64) -> Result<CatalogItem<Self::Item>, RepositoryError> {
        let catalog_item: CatalogItem<Artist> = sqlx::query_as(&format!(
            "SELECT key, metadata FROM {} WHERE key = $1",
            ARTIST_TABLE_NAME
        ))
        .bind(&id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| RepositoryError::ItemCreate)?;

        Ok(catalog_item)
    }

    async fn update(
        &mut self,
        item: CatalogItem<Self::Item>,
    ) -> Result<CatalogItem<Self::Item>, RepositoryError> {
        sqlx::query(&format!(
            "UPDATE {} SET metadata = $1 WHERE id = $2",
            ARTIST_TABLE_NAME
        ))
        .bind(&serde_json::to_string(&item.metadata).map_err(|_| RepositoryError::ItemCreate)?)
        .bind(&item.id)
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::ItemCreate)?;

        Ok(item)
    }
}
