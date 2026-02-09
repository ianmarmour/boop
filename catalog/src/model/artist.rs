use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use zvariant::Type;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
pub struct Artist {
    pub name: String,
    pub releases: Vec<i64>,
}
