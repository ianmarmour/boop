use thiserror::Error;
use tracing::info;

use crate::{
    model::{CatalogItem, artist::Artist, release::Release, track::Track},
    repository::{
        Repository, RepositoryContext, artist::ArtistFilter, release::ReleaseFilter,
        track::TrackFilter,
    },
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
    pub async fn create_track(
        &mut self,
        track: Track,
    ) -> Result<CatalogItem<Track>, TrackServiceError> {
        info!("creating track: {:?}", track);

        let created_track = self
            .repository_context
            .track
            .lock()
            .await
            .create(track.clone())
            .await
            .map_err(|e| TrackServiceError::Internal(e.into()))?;

        let related_release = self
            .repository_context
            .release
            .lock()
            .await
            .find(ReleaseFilter {
                title: track.release.clone(),
                artist: None,
            })
            .await
            .map_err(|e| TrackServiceError::Internal(e.into()))?
            .pop();

        let releated_artist = self
            .repository_context
            .artist
            .lock()
            .await
            .find(ArtistFilter {
                name: track.artist.clone(),
                track: None,
            })
            .await
            .map_err(|e| TrackServiceError::Internal(e.into()))?
            .pop();

        if let Some(release_name) = track.release.clone() {
            match related_release {
                Some(mut release_item) => {
                    release_item.metadata.add_track(&track.title);

                    self.repository_context
                        .release
                        .lock()
                        .await
                        .update(release_item)
                        .await
                        .map_err(|e| TrackServiceError::Internal(e.into()))?;
                }
                None => {
                    self.repository_context
                        .release
                        .lock()
                        .await
                        .create(Release {
                            title: release_name.clone(),
                            artist: track.artist.clone(),
                            tracks: vec![created_track.metadata.title.clone()],
                        })
                        .await
                        .map_err(|e| TrackServiceError::Internal(e.into()))?;
                }
            }
        }

        if let Some(artist_name) = track.artist {
            match releated_artist {
                Some(mut artist_item) => {
                    if let Some(release) = &track.release {
                        artist_item.metadata.add_release(release);
                    }

                    self.repository_context
                        .artist
                        .lock()
                        .await
                        .update(artist_item)
                        .await
                        .map_err(|e| TrackServiceError::Internal(e.into()))?;
                }
                None => {
                    self.repository_context
                        .artist
                        .lock()
                        .await
                        .create(Artist {
                            name: artist_name,
                            releases: Vec::new(),
                        })
                        .await
                        .map_err(|e| TrackServiceError::Internal(e.into()))?;
                }
            }
        };

        Ok(created_track)
    }

    pub async fn favorite_track(
        &mut self,
        title: &str,
    ) -> Result<CatalogItem<Track>, TrackServiceError> {
        let mut track = self.get_track(title).await?;
        track.favorite = true;

        self.repository_context
            .track
            .lock()
            .await
            .update(track)
            .await
            .map_err(|e| TrackServiceError::Internal(e.into()))
    }

    pub async fn get_track(&mut self, name: &str) -> Result<CatalogItem<Track>, TrackServiceError> {
        let tracks = self
            .repository_context
            .track
            .lock()
            .await
            .find(TrackFilter {
                name: Some(name.to_string()),
                artist: None,
                release: None,
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
