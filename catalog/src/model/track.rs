use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::{io, path::PathBuf, time::Duration};
use thiserror::Error;
use zvariant::Type;

#[derive(Debug, Error)]
pub enum TrackError {
    #[error("unable to process track file")]
    File(#[from] io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
pub struct Track {
    pub name: String,
    pub release: u64,
    pub artist: u64,
    pub file_path: PathBuf,
    pub timestamp: Duration,
}
