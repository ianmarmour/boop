use thiserror::Error;

use crate::{
    model::{CatalogItem, track::Track},
    repository::{Repository, track::TrackFilter},
    service::ServiceContext,
};

#[derive(Debug, Error)]
pub enum TrackServiceError {
    #[error("artist was not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub struct TrackService {
    context: ServiceContext,
}

impl TrackService {
    pub fn new(context: ServiceContext) -> TrackService {
        Self { context }
    }
}

impl TrackService {
    async fn get_track(&mut self, name: &str) -> Result<CatalogItem<Track>, TrackServiceError> {
        let tracks = self
            .context
            .tracks
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
}
