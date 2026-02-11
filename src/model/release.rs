use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Release {
    pub title: String,
    pub tracks: Vec<u64>,
    pub artist: String,
}
