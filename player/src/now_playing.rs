use chrono::DateTime;
use iced::{
    Alignment::Center,
    Element, Length, Task,
    widget::{Column, Container, Image, container, image, text},
};
use model::{
    CatalogItem,
    track::{self, Track},
};
use symphonia::core::units::Duration;
const ALBUM_ART_PLACEHOLDER: &[u8] = include_bytes!("../resources/album_art_placeholder.png");

#[derive(Debug, Clone)]
enum NowPlayingState {
    Playing,
    Paused,
}

#[derive(Debug, Clone)]
pub enum NowPlayingMessage {
    PlaybackStart,
    PlaybackStop,
    PlaybackComplete,
}

#[derive(Debug, Clone)]
pub struct NowPlaying {
    state: NowPlayingState,
    timestamp: Duration,
    track: Option<Track>,
}

impl Default for NowPlaying {
    fn default() -> Self {
        NowPlaying {
            state: NowPlayingState::Paused,
            timestamp: Duration::default(),
            track: None,
        }
    }
}

impl NowPlaying {
    pub fn new(track: Track) -> Self {
        NowPlaying {
            state: NowPlayingState::Paused,
            timestamp: Duration::default(),
            track: Some(track),
        }
    }

    pub fn view(&self) -> Element<NowPlayingMessage> {
        // column
        // album art
        //
        // title
        //
        // artist
        //
        // duration/elapsed

        let column = Column::new().align_x(Center);

        let album_art = Image::new(image::Handle::from_bytes(ALBUM_ART_PLACEHOLDER))
            .width(Length::Fill)
            .height(Length::Fill);

        let column = column.push(album_art);

        let column = match &self.track {
            Some(track) => column.push(text(track.name.clone())),
            None => column,
        };

        let container = Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill);

        container.into()
    }
}

async fn load_track(id: u64) -> Track {
    Track {
        id: id.clone(),
        name: format!("test-track-{}", id).to_string(),
    }
}
