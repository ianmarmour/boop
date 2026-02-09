use serde::{Deserialize, Serialize};
use sqlx::{Row, prelude::FromRow};
use zvariant::Type;

pub mod artist;
pub mod release;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct CatalogItem<T>
where
    T: Type,
{
    pub id: i64,
    pub metadata: T,
}

impl<'r, T> FromRow<'r, sqlx::any::AnyRow> for CatalogItem<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Type,
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
