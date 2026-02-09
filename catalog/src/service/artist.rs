use std::sync::Arc;

use crate::{
    model::{CatalogItem, artist::Artist},
    repository::{
        Repository,
        artist::{ArtistFilter, ArtistRepository},
    },
};
use tokio::sync::Mutex;
use zbus::fdo::Error;
use zbus::interface;

pub struct ArtistService {
    repository: Arc<Mutex<ArtistRepository>>,
}

impl ArtistService {
    pub fn new(repository: Arc<Mutex<ArtistRepository>>) -> ArtistService {
        Self { repository }
    }
}

#[interface(name = "org.boop.catalog.artist")]
impl ArtistService {
    async fn get_artist(&mut self, name: &str) -> Result<CatalogItem<Artist>, Error> {
        let artists = self
            .repository
            .lock()
            .await
            .find(ArtistFilter {
                name: Some(name.to_string()),
            })
            .await
            .map_err(|e| Error::Failed(e.to_string()))?;

        artists
            .first()
            .cloned()
            .ok_or_else(|| Error::Failed("no artist found".to_string()))
    }

    async fn list_artists(
        &mut self,
        filter: ArtistFilter,
    ) -> Result<Vec<CatalogItem<Artist>>, Error> {
        self.repository
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| Error::Failed(e.to_string()))
    }
}
