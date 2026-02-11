use thiserror::Error;

use crate::{
    model::{CatalogItem, artist::Artist},
    repository::{Repository, artist::ArtistFilter},
    service::ServiceContext,
};

#[derive(Debug, Error)]
pub enum ArtistServiceError {
    #[error("artist was not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub struct ArtistService {
    context: ServiceContext,
}

impl ArtistService {
    pub fn new(context: ServiceContext) -> ArtistService {
        Self { context }
    }
}

impl ArtistService {
    async fn get_artist(&mut self, name: &str) -> Result<CatalogItem<Artist>, ArtistServiceError> {
        let artists = self
            .context
            .artists
            .lock()
            .await
            .find(ArtistFilter {
                name: Some(name.to_string()),
            })
            .await
            .map_err(|e| ArtistServiceError::Internal(e.into()))?;

        artists
            .first()
            .cloned()
            .ok_or_else(|| ArtistServiceError::NotFound)
    }

    async fn list_artists(
        &mut self,
        filter: ArtistFilter,
    ) -> Result<Vec<CatalogItem<Artist>>, ArtistServiceError> {
        self.context
            .artists
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| ArtistServiceError::Internal(e.into()))
    }
}
