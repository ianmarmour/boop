use serde::{Deserialize, Serialize};
use sqlx::{Row, prelude::FromRow};

use crate::model::{artist::Artist, release::Release, track::Track};

pub mod artist;
pub mod release;
pub mod track;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItem<T> {
    pub id: i64,
    pub metadata: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatalogMetadata {
    Track(Track),
    Artist(Artist),
    Release(Release),
}

impl CatalogMetadata {
    pub fn display_text(&self) -> &str {
        match self {
            CatalogMetadata::Artist(artist) => &artist.name,
            CatalogMetadata::Release(release) => &release.title,
            CatalogMetadata::Track(track) => &track.title,
        }
    }
}

impl From<CatalogItem<Artist>> for CatalogItem<CatalogMetadata> {
    fn from(item: CatalogItem<Artist>) -> Self {
        CatalogItem {
            id: item.id,
            metadata: CatalogMetadata::Artist(item.metadata),
        }
    }
}

impl From<CatalogItem<Release>> for CatalogItem<CatalogMetadata> {
    fn from(item: CatalogItem<Release>) -> Self {
        CatalogItem {
            id: item.id,
            metadata: CatalogMetadata::Release(item.metadata),
        }
    }
}

impl From<CatalogItem<Track>> for CatalogItem<CatalogMetadata> {
    fn from(item: CatalogItem<Track>) -> Self {
        CatalogItem {
            id: item.id,
            metadata: CatalogMetadata::Track(item.metadata),
        }
    }
}

impl<'r, T> FromRow<'r, sqlx::any::AnyRow> for CatalogItem<T>
where
    T: for<'de> Deserialize<'de>,
{
    fn from_row(row: &'r sqlx::any::AnyRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let raw_json: String = row.try_get("metadata")?;
        let metadata: T =
            serde_json::from_str(&raw_json).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        Ok(CatalogItem { id, metadata })
    }
}
