use thiserror::Error;

use crate::{
    model::{CatalogItem, release::Release},
    repository::{Repository, RepositoryContext, release::ReleaseFilter},
};

#[derive(Debug, Error)]
pub enum ReleaseServiceError {
    #[error("release was not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct ReleaseService {
    repository_context: RepositoryContext,
}

impl ReleaseService {
    pub fn new(repository_context: RepositoryContext) -> ReleaseService {
        Self { repository_context }
    }
}

impl ReleaseService {
    pub async fn get_release(
        &mut self,
        title: &str,
    ) -> Result<CatalogItem<Release>, ReleaseServiceError> {
        let releases = self
            .repository_context
            .release
            .lock()
            .await
            .find(ReleaseFilter {
                title: Some(title.to_string()),
                artist: None,
            })
            .await
            .map_err(|e| ReleaseServiceError::Internal(e.into()))?;

        releases
            .first()
            .cloned()
            .ok_or_else(|| ReleaseServiceError::NotFound)
    }

    pub async fn list_releases(
        &mut self,
        filter: ReleaseFilter,
    ) -> Result<Vec<CatalogItem<Release>>, ReleaseServiceError> {
        self.repository_context
            .release
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| ReleaseServiceError::Internal(e.into()))
    }
}
