pub mod artist;
pub mod release;
pub mod track;

#[derive(Debug, Clone)]
pub enum CatalogItem {
    Artist(artist::Artist),
    Release(release::Release),
    Track(track::Track),
}
