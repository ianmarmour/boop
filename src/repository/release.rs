use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    model::{CatalogItem, release::Release},
    repository::{Repository, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct ReleaseRepository {
    pool: SqlitePool,
}

impl ReleaseRepository {
    pub async fn new(pool: SqlitePool) -> Result<Self, RepositoryError> {
        let mut repository = Self { pool };
        repository.setup().await?;
        Ok(repository)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ReleaseFilter {
    pub title: Option<String>,
    pub artist: Option<String>,
}

#[async_trait]
impl Repository for ReleaseRepository {
    const TABLE_NAME: &'static str = "releases";

    type Item = Release;
    type Filter = ReleaseFilter;

    async fn setup(&mut self) -> Result<(), RepositoryError> {
        sqlx::query(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                favorite BOOL NOT NULL DEFAULT FALSE,
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
            "INSERT INTO {} (metadata) VALUES ($1) RETURNING id, favorite, metadata",
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
            "SELECT id, favorite, metadata FROM {} WHERE id = $1",
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
            "UPDATE {} SET metadata = $1, favorite = $2 WHERE id = $3",
            Self::TABLE_NAME
        ))
        .bind(
            &serde_json::to_string(&item.metadata)
                .map_err(|e| RepositoryError::ItemCreate(e.to_string()))?,
        )
        .bind(&item.favorite)
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
        let mut sql = format!("SELECT id, favorite, metadata FROM {}", Self::TABLE_NAME);
        let mut conditions: Vec<String> = Vec::new();

        if filter.title.is_some() {
            conditions.push(format!("metadata->>'title' LIKE ${}", conditions.len() + 1));
        }
        if filter.artist.is_some() {
            conditions.push(format!(
                "metadata->>'artist' LIKE ${}",
                conditions.len() + 1
            ));
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let mut query = sqlx::query_as::<_, CatalogItem<Self::Item>>(&sql);

        if let Some(title) = &filter.title {
            query = query.bind(format!("%{}%", title));
        }
        if let Some(artist) = &filter.artist {
            query = query.bind(format!("%{}%", artist));
        }

        let all: Vec<(String,)> = sqlx::query_as("SELECT metadata FROM releases")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        println!("All releases: {:?}", all);

        query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::ItemRead(e.to_string()))
    }
}
