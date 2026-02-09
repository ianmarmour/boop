use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use zvariant::Type;

use crate::{
    model::{CatalogItem, artist::Artist},
    repository::{Repository, RepositoryError},
};

pub struct ArtistRepository {
    pool: AnyPool,
}

impl ArtistRepository {
    pub fn new(pool: AnyPool) -> Self {
        ArtistRepository { pool }
    }
}

#[derive(Default, Serialize, Deserialize, Type)]
pub struct ArtistFilter {
    pub name: Option<String>,
}

#[async_trait]
impl Repository for ArtistRepository {
    type Item = Artist;
    type Filter = ArtistFilter;
    const TABLE_NAME: &'static str = "artists";

    async fn create(
        &mut self,
        item: Self::Item,
    ) -> Result<CatalogItem<Self::Item>, RepositoryError> {
        let catalog_item: CatalogItem<Artist> = sqlx::query_as(&format!(
            "INSERT INTO {} (metadata) VALUES ($1) RETURNING id, metadata",
            Self::TABLE_NAME
        ))
        .bind(serde_json::to_string(&item).map_err(|_| RepositoryError::ItemCreate)?)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| RepositoryError::ItemCreate)?;

        Ok(catalog_item)
    }

    async fn read(&mut self, id: &i64) -> Result<CatalogItem<Self::Item>, RepositoryError> {
        let catalog_item: CatalogItem<Artist> = sqlx::query_as(&format!(
            "SELECT id, metadata FROM {} WHERE id = $1",
            Self::TABLE_NAME
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
            Self::TABLE_NAME
        ))
        .bind(&serde_json::to_string(&item.metadata).map_err(|_| RepositoryError::ItemCreate)?)
        .bind(&item.id)
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::ItemCreate)?;

        Ok(item)
    }

    async fn delete(&mut self, id: &i64) -> Result<(), RepositoryError> {
        sqlx::query(&format!("DELETE FROM {} WHERE id = $1", Self::TABLE_NAME))
            .bind(&id)
            .execute(&self.pool)
            .await
            .map_err(|_| RepositoryError::ItemCreate)?;

        Ok(())
    }

    async fn find(
        &self,
        filter: Self::Filter,
    ) -> Result<Vec<CatalogItem<Self::Item>>, RepositoryError> {
        let mut sql = format!("SELECT id, metadata FROM {}", Self::TABLE_NAME);
        let mut conditions: Vec<String> = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(name) = &filter.name {
            conditions.push("metadata->>'name' LIKE ?".into());
            params.push(format!("%{}%", name));
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let mut query = sqlx::query_as::<_, CatalogItem<Artist>>(&sql);
        for param in &params {
            query = query.bind(param);
        }

        query
            .fetch_all(&self.pool)
            .await
            .map_err(|_| RepositoryError::ItemRead)
    }
}
