use std::{path::PathBuf, sync::Arc};

use thiserror::Error;
use tokio::sync::Mutex;

use crate::{
    repository::{RepositoryContext, RepositoryError},
    service::{artist::ArtistService, release::ReleaseService, track::TrackService},
};

pub mod artist;
pub mod release;
pub mod track;

#[derive(Debug, Clone, Error)]
pub enum CatalogError {
    #[error("unknown error occured")]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Catalog {
    pub artist: Arc<Mutex<ArtistService>>,
    pub release: Arc<Mutex<ReleaseService>>,
    pub track: Arc<Mutex<TrackService>>,
}

impl Catalog {
    pub async fn new(repository_context: RepositoryContext) -> Result<Self, CatalogError> {
        Ok(Self {
            artist: Arc::new(Mutex::new(ArtistService::new(repository_context.clone()))),
            release: Arc::new(Mutex::new(ReleaseService::new(repository_context.clone()))),
            track: Arc::new(Mutex::new(TrackService::new(repository_context.clone()))),
        })
    }

    pub async fn sync(path: PathBuf) -> Result<(), CatalogError> {
        let mut tracks = vec![];
        let mut dirs = vec![path.clone()];

        while let Some(dir) = dirs.pop() {
            let mut entries = tokio::fs::read_dir(dir)
                .await
                .map_err(|_| CatalogError::Unknown)?;
            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|_| CatalogError::Unknown)?
            {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("mp3") {
                    tracks.push(path);
                }
            }
        }

        Ok(())
    }
}
