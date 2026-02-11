use thiserror::Error;

use crate::{
    model::{CatalogItem, track::Track},
    repository::{Repository, RepositoryContext, track::TrackFilter},
};

#[derive(Debug, Error)]
pub enum TrackServiceError {
    #[error("artist was not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct TrackService {
    repository_context: RepositoryContext,
}

impl TrackService {
    pub fn new(repository_context: RepositoryContext) -> TrackService {
        Self { repository_context }
    }
}

impl TrackService {
    pub async fn get_track(&mut self, name: &str) -> Result<CatalogItem<Track>, TrackServiceError> {
        let tracks = self
            .repository_context
            .track
            .lock()
            .await
            .find(TrackFilter {
                name: Some(name.to_string()),
            })
            .await
            .map_err(|e| TrackServiceError::Internal(e.into()))?;

        tracks
            .first()
            .cloned()
            .ok_or_else(|| TrackServiceError::NotFound)
    }

    pub async fn list_tracks(
        &mut self,
        filter: TrackFilter,
    ) -> Result<Vec<CatalogItem<Track>>, TrackServiceError> {
        self.repository_context
            .track
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| TrackServiceError::Internal(e.into()))
    }
}
