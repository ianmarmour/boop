use crate::{
    model::{CatalogItem, artist::Artist},
    repository::{Repository, RepositoryError},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct ArtistRepository {
    pool: SqlitePool,
}

impl ArtistRepository {
    pub async fn new(pool: SqlitePool) -> Result<Self, RepositoryError> {
        let mut repository = Self { pool };
        repository.setup().await?;
        Ok(repository)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ArtistFilter {
    pub name: Option<String>,
    pub track: Option<String>,
}

#[async_trait]
impl Repository for ArtistRepository {
    const TABLE_NAME: &'static str = "artists";

    type Item = Artist;
    type Filter = ArtistFilter;

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
        let catalog_item: CatalogItem<Artist> = sqlx::query_as(&format!(
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
        let catalog_item: CatalogItem<Artist> = sqlx::query_as(&format!(
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

        if filter.name.is_some() {
            conditions.push("metadata->>'name' LIKE ?".into());
        }
        if filter.track.is_some() {
            conditions.push(
                "EXISTS (
                    SELECT 1 FROM json_each(metadata, '$.tracks')
                    WHERE CAST(json_each.value AS INTEGER) = ?
                )"
                .into(),
            );
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let mut query = sqlx::query_as::<_, CatalogItem<Self::Item>>(&sql);

        if let Some(name) = &filter.name {
            query = query.bind(format!("%{}%", name));
        }
        if let Some(track_id) = &filter.track {
            query = query.bind(track_id);
        }

        query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::ItemRead(e.to_string()))
    }
}
