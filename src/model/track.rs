use std::{path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fs::File;
use symphonia::core::{
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::{MetadataOptions, StandardTagKey},
    probe::Hint,
};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum TrackError {
    #[error("track is missing a required tag")]
    TagMissing,
    #[error("unknown error occured")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Track {
    pub title: String,
    pub release: Option<String>,
    pub artist: Option<String>,
    pub path: PathBuf,
    pub duration: Duration,
}

impl Track {
    pub fn from_path(path: PathBuf) -> Result<Self, TrackError> {
        let file = File::open(&path).map_err(|_| TrackError::Unknown)?;
        let media_stream = MediaSourceStream::new(Box::new(file), Default::default());

        let probe_result = symphonia::default::get_probe()
            .format(
                &Hint::default(),
                media_stream,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            )
            .map_err(|_| TrackError::Unknown)?;

        let mut format = probe_result.format;

        let mut title = None;
        let mut artist = None;
        let mut album = None;
        let duration = format.default_track().and_then(|track| {
            let time_base = track.codec_params.time_base?;
            let n_frames = track.codec_params.n_frames?;

            // Skip if values seem invalid
            if n_frames == 0 {
                return None;
            }

            let time = time_base.calc_time(n_frames);

            // Guard against negative or huge values
            if time.frac.is_nan() || time.frac.is_infinite() || time.frac < 0.0 {
                return Some(Duration::from_secs(time.seconds));
            }

            Some(Duration::from_secs(time.seconds) + Duration::from_secs_f64(time.frac))
        });

        // Read the files metadata tags.
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

        Ok(Track {
            title: title.ok_or_else(|| TrackError::TagMissing)?,
            artist: Some(artist.ok_or_else(|| TrackError::TagMissing)?),
            release: Some(album.ok_or_else(|| TrackError::TagMissing)?),
            duration: duration.ok_or_else(|| TrackError::Unknown)?,
            path,
        })
    }

    pub fn open(&self) -> Result<MediaSourceStream, TrackError> {
        let file = File::open(&self.path).map_err(|_| TrackError::Unknown)?;
        Ok(MediaSourceStream::new(Box::new(file), Default::default()))
    }
}
