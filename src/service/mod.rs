use std::sync::Arc;

use sqlx::AnyPool;
use tokio::sync::Mutex;

use crate::repository::{
    RepositoryError, artist::ArtistRepository, release::ReleaseRepository, track::TrackRepository,
};

pub mod artist;
pub mod release;
pub mod track;

#[derive(Clone)]
pub struct ServiceContext {
    pub artists: Arc<Mutex<ArtistRepository>>,
    pub releases: Arc<Mutex<ReleaseRepository>>,
    pub tracks: Arc<Mutex<TrackRepository>>,
}

impl ServiceContext {
    pub async fn new(pool: AnyPool) -> Result<Self, RepositoryError> {
        Ok(Self {
            artists: Arc::new(Mutex::new(ArtistRepository::new(pool.clone()).await?)),
            releases: Arc::new(Mutex::new(ReleaseRepository::new(pool.clone()).await?)),
            tracks: Arc::new(Mutex::new(TrackRepository::new(pool.clone()).await?)),
        })
    }
}
