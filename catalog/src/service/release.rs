use crate::{
    ApplicationState,
    model::{CatalogItem, release::Release},
    repository::{Repository, release::ReleaseFilter},
};
use zbus::fdo::Error;
use zbus::interface;

pub struct ReleaseService {
    application_state: ApplicationState,
}

impl ReleaseService {
    pub fn new(application_state: ApplicationState) -> ReleaseService {
        Self { application_state }
    }
}

#[interface(name = "org.boop.catalog.release")]
impl ReleaseService {
    async fn get_release(&mut self, name: &str) -> Result<CatalogItem<Release>, Error> {
        let releases = self
            .application_state
            .releases
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
        self.application_state
            .releases
            .lock()
            .await
            .find(filter)
            .await
            .map_err(|e| Error::Failed(e.to_string()))
    }
}
