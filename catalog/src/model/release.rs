use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use zvariant::Type;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
pub struct Release {
    pub id: u64,
    pub name: String,
    pub tracks: Vec<u64>,
    pub artist: u64,
}
