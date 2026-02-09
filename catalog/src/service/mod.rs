use std::sync::{Arc, Mutex};

use sqlx::AnyPool;

use crate::repository::{RepositoryError, artist::ArtistRepository, release::ReleaseRepository};

pub mod artist;
pub mod release;

pub struct Application {
    pub artist_repository: Arc<Mutex<ArtistRepository>>,
    pub release_repository: Arc<Mutex<ReleaseRepository>>,
}

impl Application {
    pub async fn new(pool: AnyPool) -> Result<Self, RepositoryError> {
        Ok(Self {
            artist_repository: Arc::new(Mutex::new(ArtistRepository::new(pool.clone()).await?)),
            release_repository: Arc::new(Mutex::new(ReleaseRepository::new(pool.clone()).await?)),
        })
    }
}
