use zbus::{Result, proxy};

use crate::{
    model::{CatalogItem, artist::Artist},
    repository::artist::ArtistFilter,
};

#[proxy(
    interface = "org.boop.catalog.artist",
    default_service = "org.boop.catalog",
    default_path = "/org/boop/catalog/artist"
)]
pub trait ArtistService {
    async fn get_artist(&self, name: &str) -> Result<CatalogItem<Artist>>;
    async fn list_artists(&self, filter: ArtistFilter) -> Result<Vec<CatalogItem<Artist>>>;
}
