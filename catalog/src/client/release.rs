use zbus::{Result, proxy};

use crate::{
    model::{CatalogItem, release::Release},
    repository::release::ReleaseFilter,
};

#[proxy(
    interface = "org.boop.catalog.release",
    default_service = "org.boop.catalog",
    default_path = "/org/boop/catalog/release"
)]
pub trait ReleaseService {
    async fn get_release(&self, name: &str) -> Result<CatalogItem<Release>>;
    async fn list_releases(&self, filter: ReleaseFilter) -> Result<Vec<CatalogItem<Release>>>;
}
