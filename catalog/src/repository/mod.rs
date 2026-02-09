use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zvariant::Type;

use crate::model::CatalogItem;

pub mod artist;

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
    type Filter: Default + Serialize + for<'de> Deserialize<'de> + Type;
    const TABLE_NAME: &'static str;

    async fn create(
        &mut self,
        item: Self::Item,
    ) -> Result<CatalogItem<Self::Item>, RepositoryError>;
    async fn read(&mut self, id: &i64) -> Result<CatalogItem<Self::Item>, RepositoryError>;
    async fn update(
        &mut self,
        item: CatalogItem<Self::Item>,
    ) -> Result<CatalogItem<Self::Item>, RepositoryError>;
    async fn delete(&mut self, id: &i64) -> Result<(), RepositoryError>;
    async fn find(
        &self,
        filter: Self::Filter,
    ) -> Result<Vec<CatalogItem<Self::Item>>, RepositoryError>;
}
