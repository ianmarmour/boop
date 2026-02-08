pub mod artist;
pub mod release;
pub mod track;

pub trait Id {
    fn id(self) -> u64;
}

#[derive(Debug, Clone)]
pub enum CatalogItem {
    Artist(artist::Artist),
    Release(release::Release),
    Track(track::Track),
}

#[derive(Debug, Clone)]

pub enum CatalogItemKind {
    Artist,
    Release,
    Track,
}

impl Id for CatalogItem {
    fn id(self) -> u64 {
        match self {
            Self::Artist(a) => a.id,
            Self::Release(r) => r.id,
            Self::Track(t) => t.id,
        }
    }
}
