use thiserror::Error;

use crate::{
    model::{CatalogItem, artist::Artist},
    repository::{Repository, RepositoryContext, artist::ArtistFilter},
};

#[derive(Debug, Error)]
pub enum ArtistServiceError {
    #[error("artist was not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct ArtistService {
    repository_context: RepositoryContext,
}

impl ArtistService {
    pub fn new(repository_context: RepositoryContext) -> ArtistService {
        Self { repository_context }
    }
}

impl ArtistService {
    pub async fn get_artist(
        &mut self,
        name: &str,
    ) -> Result<CatalogItem<Artist>, ArtistServiceError> {
        let artists = self
            .repository_context
            .artist
            .lock()
            .await
            .find(ArtistFilter {
                name: Some(name.to_string()),
                track: None,
            })
            .await
            .map_err(|e| ArtistServiceError::Internal(e.into()))?;

        artists
            .first()
            .cloned()
            .ok_or_else(|| ArtistServiceError::NotFound)
    }

    pub async fn list_artists(
        &mut self,
        filter: ArtistFilter,
    ) -> Result<Vec<CatalogItem<Artist>>, ArtistServiceError> {
        self.repository_context
            .artist
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| ArtistServiceError::Internal(e.into()))
    }
}
