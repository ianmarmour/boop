use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;
use zvariant::Type;

use crate::model::CatalogItem;

pub mod artist;

pub struct RepositoryPaginatedResponse<T> {
    items: Vec<T>,
    next_token: Option<String>,
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("unable to create item")]
    ItemCreate,
    #[error("unable to read item")]
    ItemRead,
}

#[async_trait]
pub trait Repository {
    type Item: Serialize + for<'de> Deserialize<'de> + Type;

    async fn create(
        &mut self,
        item: Self::Item,
    ) -> Result<CatalogItem<Self::Item>, RepositoryError>;
    async fn read(&mut self, id: &i64) -> Result<CatalogItem<Self::Item>, RepositoryError>;
    async fn update(
        &mut self,
        item: CatalogItem<Self::Item>,
    ) -> Result<CatalogItem<Self::Item>, RepositoryError>;
}
