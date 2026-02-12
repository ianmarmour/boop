use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;

use crate::{
    model::{CatalogItem, track::Track},
    repository::{Repository, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct TrackRepository {
    pool: AnyPool,
}

impl TrackRepository {
    pub async fn new(pool: AnyPool) -> Result<Self, RepositoryError> {
        let mut repository = Self { pool };
        repository.setup().await?;
        Ok(repository)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct TrackFilter {
    pub name: Option<String>,
    pub release: Option<String>,
    pub artist: Option<String>,
}

#[async_trait]
impl Repository for TrackRepository {
    const TABLE_NAME: &'static str = "tracks";

    type Item = Track;
    type Filter = TrackFilter;

    async fn setup(&mut self) -> Result<(), RepositoryError> {
        sqlx::query(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                metadata TEXT NOT NULL
            )",
            Self::TABLE_NAME
        ))
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::Setup)?;

        sqlx::query(&format!(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_{}_unique ON {}(
                (metadata->>'title'),
                (metadata->>'artist'),
                (metadata->>'release')
            )",
            Self::TABLE_NAME,
            Self::TABLE_NAME
        ))
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::Setup)?;

        Ok(())
    }

    async fn create(
        &mut self,
        item: Self::Item,
    ) -> Result<CatalogItem<Self::Item>, RepositoryError> {
        let catalog_item: CatalogItem<Self::Item> = sqlx::query_as(&format!(
            "INSERT INTO {} (metadata) VALUES ($1) RETURNING id, metadata",
            Self::TABLE_NAME
        ))
        .bind(serde_json::to_string(&item).map_err(|e| RepositoryError::ItemCreate(e.to_string()))?)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::ItemCreate(e.to_string()))?;

        Ok(catalog_item)
    }

    async fn read(&mut self, id: &i64) -> Result<CatalogItem<Self::Item>, RepositoryError> {
        let catalog_item: CatalogItem<Self::Item> = sqlx::query_as(&format!(
            "SELECT id, metadata FROM {} WHERE id = $1",
            Self::TABLE_NAME
        ))
        .bind(&id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::ItemCreate(e.to_string()))?;

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
        .bind(
            &serde_json::to_string(&item.metadata)
                .map_err(|e| RepositoryError::ItemCreate(e.to_string()))?,
        )
        .bind(&item.id)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::ItemCreate(e.to_string()))?;

        Ok(item)
    }

    async fn delete(&mut self, id: &i64) -> Result<(), RepositoryError> {
        sqlx::query(&format!("DELETE FROM {} WHERE id = $1", Self::TABLE_NAME))
            .bind(&id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::ItemCreate(e.to_string()))?;

        Ok(())
    }

    async fn find(
        &self,
        filter: Self::Filter,
    ) -> Result<Vec<CatalogItem<Self::Item>>, RepositoryError> {
        let mut sql = format!("SELECT id, metadata FROM {}", Self::TABLE_NAME);
        let mut conditions: Vec<String> = Vec::new();

        if filter.name.is_some() {
            conditions.push("metadata->>'name' LIKE ?".into());
        }
        if filter.artist.is_some() {
            conditions.push("metadata->>'artist' LIKE ?".into());
        }
        if filter.release.is_some() {
            conditions.push("metadata->>'release' LIKE ?".into());
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let mut query = sqlx::query_as::<_, CatalogItem<Self::Item>>(&sql);

        if let Some(name) = &filter.name {
            query = query.bind(format!("%{}%", name));
        }
        if let Some(artist) = &filter.artist {
            query = query.bind(format!("%{}%", artist));
        }
        if let Some(release) = &filter.release {
            query = query.bind(format!("%{}%", release));
        }

        query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::ItemRead(e.to_string()))
    }
}
