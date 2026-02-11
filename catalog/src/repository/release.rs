use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use zvariant::Type;

use crate::{
    model::{CatalogItem, release::Release},
    repository::{Repository, RepositoryError},
};

pub struct ReleaseRepository {
    pool: AnyPool,
}

impl ReleaseRepository {
    pub async fn new(pool: AnyPool) -> Result<Self, RepositoryError> {
        let mut repository = Self { pool };
        repository.setup().await?;
        Ok(repository)
    }
}

#[derive(Default, Serialize, Deserialize, Type)]
pub struct ReleaseFilter {
    pub name: Option<String>,
    pub artist: Option<i64>,
}

#[async_trait]
impl Repository for ReleaseRepository {
    type Item = Release;
    type Filter = ReleaseFilter;
    const TABLE_NAME: &'static str = "releases";

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
        .expect("failed to create artists table");

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
        let mut params: Vec<String> = Vec::new();

        if let Some(name) = &filter.name {
            conditions.push("metadata->>'name' LIKE ?".into());
            params.push(format!("%{}%", name));
        }

        if let Some(artist) = &filter.artist {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM json_each(metadata->>'artists') WHERE value = ?)"
            ));
            params.push(format!("%{}%", artist));
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let mut query = sqlx::query_as::<_, CatalogItem<Release>>(&sql);
        for param in &params {
            query = query.bind(param);
        }

        query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::ItemRead(e.to_string()))
    }
}
