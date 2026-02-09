use std::sync::Arc;

use tokio::sync::Mutex;
use zbus::fdo::Error;
use zbus::interface;

use crate::{
    model::{CatalogItem, artist::Artist},
    repository::{Repository, artist::ArtistRepository},
};

pub struct ArtistService {
    repository: Arc<Mutex<ArtistRepository>>,
}

impl ArtistService {
    pub fn new(repository: Arc<Mutex<ArtistRepository>>) -> ArtistService {
        Self { repository }
    }
}

#[interface(name = "org.boop.artist")]
impl ArtistService {
    async fn create(&mut self, item: CatalogItem<Artist>) -> Result<CatalogItem<Artist>, Error> {
        self.repository
            .lock()
            .await
            .create(item)
            .await
            .map_err(|e| Error::Failed(e.to_string()))
    }

    async fn read(&mut self, id: i64) -> Result<CatalogItem<Artist>, Error> {
        self.repository
            .lock()
            .await
            .read(&id)
            .await
            .map_err(|e| Error::Failed(e.to_string()))
    }

    async fn update(&mut self, item: CatalogItem<Artist>) -> Result<CatalogItem<Artist>, Error> {
        self.repository
            .lock()
            .await
            .update(item)
            .await
            .map_err(|e| Error::Failed(e.to_string()))
    }
}
