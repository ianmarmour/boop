use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Release {
    pub id: u64,
    pub name: String,
    pub tracks: Vec<u64>,
    pub artist: u64,
}
