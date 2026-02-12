use std::fmt::Debug;

use crate::{audio::AudioHandle, model::track::Track};
use iced::{
    Alignment::Center,
    Element, Length, Task,
    keyboard::Key,
    widget::{Column, Container, Image, image, text},
};

const ALBUM_ART_PLACEHOLDER: &[u8] = include_bytes!("album_art.png");

#[derive(Debug, Default, Clone)]
enum PlayerState {
    #[default]
    Paused,
    Playing,
}

#[derive(Debug, Clone)]
pub enum PlayerMessage {
    Play,
    Pause,
    Load(Track),
    Input(Key),
    Error(String),
}

pub struct Player {
    state: PlayerState,
    track: Option<Track>,
    audio: Option<AudioHandle>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            state: PlayerState::Paused,
            track: None,
            audio: None,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            state: PlayerState::default(),
            track: None,
            audio: None,
        }
    }

    pub fn view(&self) -> Element<PlayerMessage> {
        let album_art = Image::new(image::Handle::from_bytes(ALBUM_ART_PLACEHOLDER))
            .width(Length::Fixed(80.0))
            .height(Length::Fixed(80.0));

        let content = match &self.track {
            Some(track) => Column::new()
                .align_x(Center)
                .push(album_art)
                .push(text(track.title.clone()))
                .push(text(track.artist.clone().unwrap_or_default())),
            None => Column::new()
                .align_x(Center)
                .push(album_art)
                .push(text("No track loaded")),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: PlayerMessage) -> Task<PlayerMessage> {
        match message {
            PlayerMessage::Load(track) => {
                let handle = AudioHandle::new();
                handle.load(&track.path);

                self.track = Some(track.clone());
                self.audio = Some(handle);

                Task::done(PlayerMessage::Play)
            }
            PlayerMessage::Play if self.audio.is_some() => {
                self.state = PlayerState::Playing;
                self.audio.as_ref().unwrap().play();
                Task::none()
            }
            PlayerMessage::Pause if self.audio.is_some() => {
                self.state = PlayerState::Paused;
                self.audio.as_ref().unwrap().pause();
                Task::none()
            }
            _ => todo!(),
        }
    }
}
