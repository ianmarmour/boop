use thiserror::Error;

use crate::{
    model::{CatalogItem, release::Release},
    repository::{Repository, release::ReleaseFilter},
    service::ServiceContext,
};

#[derive(Debug, Error)]
pub enum ReleaseServiceError {
    #[error("release was not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub struct ReleaseService {
    context: ServiceContext,
}

impl ReleaseService {
    pub fn new(context: ServiceContext) -> ReleaseService {
        Self { context }
    }
}

impl ReleaseService {
    async fn get_release(
        &mut self,
        name: &str,
    ) -> Result<CatalogItem<Release>, ReleaseServiceError> {
        let releases = self
            .context
            .releases
            .lock()
            .await
            .find(ReleaseFilter {
                name: Some(name.to_string()),
                artist: None,
            })
            .await
            .map_err(|e| ReleaseServiceError::Internal(e.into()))?;

        releases
            .first()
            .cloned()
            .ok_or_else(|| ReleaseServiceError::NotFound)
    }

    async fn list_releases(
        &mut self,
        filter: ReleaseFilter,
    ) -> Result<Vec<CatalogItem<Release>>, ReleaseServiceError> {
        self.context
            .releases
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| ReleaseServiceError::Internal(e.into()))
    }
}
