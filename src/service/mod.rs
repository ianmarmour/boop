use std::{path::PathBuf, sync::Arc};

use tracing::{debug, info};

use thiserror::Error;
use tokio::sync::Mutex;
use tracing_subscriber::field::debug;

use crate::{
    model::track::Track,
    repository::RepositoryContext,
    service::{artist::ArtistService, release::ReleaseService, track::TrackService},
};

pub mod artist;
pub mod release;
pub mod track;

#[derive(Debug, Error)]
pub enum CatalogServiceError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("unknown error occured")]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CatalogService {
    pub artist: Arc<Mutex<ArtistService>>,
    pub release: Arc<Mutex<ReleaseService>>,
    pub track: Arc<Mutex<TrackService>>,
}

impl CatalogService {
    pub async fn new(context: RepositoryContext) -> Result<Self, CatalogServiceError> {
        Ok(Self {
            artist: Arc::new(Mutex::new(ArtistService::new(context.clone()))),
            release: Arc::new(Mutex::new(ReleaseService::new(context.clone()))),
            track: Arc::new(Mutex::new(TrackService::new(context.clone()))),
        })
    }

    /// Synchronizes the files contained within a `PathBuf`'s directory structure.
    pub async fn sync(&self, path: PathBuf) -> Result<(), CatalogServiceError> {
        let mut track_paths = vec![];
        let mut dirs = vec![path.clone()];

        while let Some(dir) = dirs.pop() {
            let mut entries = tokio::fs::read_dir(dir)
                .await
                .map_err(|e| CatalogServiceError::Internal(e.into()))?;

            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|e| CatalogServiceError::Internal(e.into()))?
            {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("flac") {
                    track_paths.push(path);
                }
            }
        }

        for path in track_paths {
            let track = Track::from_path(path.clone()).expect("error creating track");
            let _ = self.track.lock().await.create_track(track).await;
        }

        Ok(())
    }
}
