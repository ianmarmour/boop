use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Artist {
    pub name: String,
    pub releases: Vec<String>,
}

impl Artist {
    pub fn add_release(&mut self, title: &str) {
        if self.releases.iter().find(|r| **r == title).is_none() {
            self.releases.push(title.to_string())
        }
    }
}
