use zbus::proxy;

use crate::{
    model::{CatalogItem, artist::Artist},
    repository::artist::ArtistFilter,
};

#[proxy(
    interface = "org.boop.artist",
    default_service = "org.boop.artist",
    default_path = "/org/boop/artist"
)]
pub trait ArtistService {
    async fn get_artist(&self, name: &str) -> zbus::Result<CatalogItem<Artist>>;
    async fn list_artists(&self, filter: ArtistFilter) -> zbus::Result<Vec<CatalogItem<Artist>>>;
}
