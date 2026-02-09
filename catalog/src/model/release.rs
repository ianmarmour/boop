use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use zvariant::Type;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
pub struct Release {
    pub name: String,
    pub artist: u64,
    pub tracks: Vec<u64>,
}
