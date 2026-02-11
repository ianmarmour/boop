use serde::{Deserialize, Serialize};
use sqlx::{Row, prelude::FromRow};

use crate::model::{artist::Artist, release::Release, track::Track};

pub mod artist;
pub mod release;
pub mod track;

#[derive(Debug, Clone)]
pub enum CatalogItemKind {
    Track(Track),
    Release(Release),
    Artist(Artist),
}

impl CatalogItemKind {
    pub fn id(&self) -> u64 {
        self.id()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct CatalogItem<T> {
    pub id: i64,
    pub metadata: T,
}

impl<'r, T> FromRow<'r, sqlx::any::AnyRow> for CatalogItem<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn from_row(row: &'r sqlx::any::AnyRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let raw_json: String = row.try_get("item")?;
        let metadata: T =
            serde_json::from_str(&raw_json).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(CatalogItem {
            id,
            metadata: metadata,
        })
    }
}
