use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{
    model::CatalogItem,
    repository::{artist::ArtistRepository, release::ReleaseRepository, track::TrackRepository},
};

pub mod artist;
pub mod release;
pub mod track;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("unable to setup repository")]
    Setup,
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

#[derive(Debug, Clone)]
pub struct RepositoryContext {
    pub artist: Arc<Mutex<ArtistRepository>>,
    pub release: Arc<Mutex<ReleaseRepository>>,
    pub track: Arc<Mutex<TrackRepository>>,
}

impl RepositoryContext {
    pub async fn new(pool: SqlitePool) -> Result<Self, RepositoryError> {
        Ok(Self {
            artist: Arc::new(Mutex::new(ArtistRepository::new(pool.clone()).await?)),
            release: Arc::new(Mutex::new(ReleaseRepository::new(pool.clone()).await?)),
            track: Arc::new(Mutex::new(TrackRepository::new(pool.clone()).await?)),
        })
    }
}
