use crate::{
    ApplicationState,
    model::{CatalogItem, artist::Artist},
    repository::{Repository, artist::ArtistFilter},
};
use zbus::fdo::Error;
use zbus::interface;

pub struct ArtistService {
    application_state: ApplicationState,
}

impl ArtistService {
    pub fn new(application_state: ApplicationState) -> ArtistService {
        Self { application_state }
    }
}

#[interface(name = "org.boop.catalog.artist")]
impl ArtistService {
    async fn get_artist(&mut self, name: &str) -> Result<CatalogItem<Artist>, Error> {
        let artists = self
            .application_state
            .artists
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
        self.application_state
            .artists
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| Error::Failed(e.to_string()))
    }
}
