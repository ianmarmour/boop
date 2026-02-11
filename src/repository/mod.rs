use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::CatalogItem;

pub mod artist;
pub mod release;
pub mod track;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("unable to create item: {0}")]
    ItemCreate(String),
    #[error("unable to read item: {0}")]
    ItemRead(String),
}

#[async_trait]
pub trait Repository {
    const TABLE_NAME: &'static str;

    type Item: Serialize + for<'de> Deserialize<'de>;
    type Filter: Default + Serialize + for<'de> Deserialize<'de>;

    async fn setup(&mut self) -> Result<(), RepositoryError>;
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
