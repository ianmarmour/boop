use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use symphonia::core::{io::MediaSourceStream, meta::StandardTagKey, probe::Hint};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum TrackError {
    #[error("unknown error occured")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub release: String,
    pub artist: String,
}

impl Track {
    pub fn from_path(path: PathBuf) -> Result<Self, TrackError> {
        let file = std::fs::File::open(&path).map_err(|_| TrackError::Unknown)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &Default::default(), &Default::default())
            .map_err(|_| TrackError::Unknown)?;

        let mut format = probed.format;

        let mut title = None;
        let mut artist = None;
        let mut album = None;

        // Read metadata tags
        if let Some(metadata) = format.metadata().current() {
            for tag in metadata.tags() {
                match tag.std_key {
                    Some(StandardTagKey::TrackTitle) => title = Some(tag.value.to_string()),
                    Some(StandardTagKey::Artist) => artist = Some(tag.value.to_string()),
                    Some(StandardTagKey::Album) => album = Some(tag.value.to_string()),
                    _ => {}
                }
            }
        }

        // Calculate duration from the default track
        let duration = format.default_track().and_then(|t| {
            let tb = t.codec_params.time_base?;
            let frames = t.codec_params.n_frames?;
            let time = tb.calc_time(frames);
            Some(time.seconds)
        });

        Ok(Track {
            id: title.clone().ok_or_else(|| TrackError::Unknown)?,
            name: title.ok_or_else(|| TrackError::Unknown)?,
            artist: artist.ok_or_else(|| TrackError::Unknown)?,
            release: album.ok_or_else(|| TrackError::Unknown)?,
        })
    }
}
