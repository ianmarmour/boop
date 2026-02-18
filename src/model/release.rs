use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Release {
    pub title: String,
    pub tracks: Vec<String>,
    pub artist: Option<String>,
}

impl Release {
    pub fn add_track(&mut self, title: &str) {
        if self.tracks.iter().find(|t| **t == title).is_none() {
            self.tracks.push(title.to_string())
        }
    }
}
