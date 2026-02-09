use std::sync::Arc;

use crate::{
    model::{CatalogItem, release::Release},
    repository::{
        Repository,
        release::{ReleaseFilter, ReleaseRepository},
    },
};
use tokio::sync::Mutex;
use zbus::fdo::Error;
use zbus::interface;

pub struct ReleaseService {
    repository: Arc<Mutex<ReleaseRepository>>,
}

impl ReleaseService {
    pub fn new(repository: Arc<Mutex<ReleaseRepository>>) -> ReleaseService {
        Self { repository }
    }
}

#[interface(name = "org.boop.catalog.release")]
impl ReleaseService {
    async fn get_release(&mut self, name: &str) -> Result<CatalogItem<Release>, Error> {
        let releases = self
            .repository
            .lock()
            .await
            .find(ReleaseFilter {
                name: Some(name.to_string()),
            })
            .await
            .map_err(|e| Error::Failed(e.to_string()))?;

        releases
            .first()
            .cloned()
            .ok_or_else(|| Error::Failed("no release found".to_string()))
    }

    async fn list_releases(
        &mut self,
        filter: ReleaseFilter,
    ) -> Result<Vec<CatalogItem<Release>>, Error> {
        self.repository
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| Error::Failed(e.to_string()))
    }
}
