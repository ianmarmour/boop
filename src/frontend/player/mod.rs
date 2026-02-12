use std::{fmt::Debug, sync::LazyLock, time::Duration};

use crate::{audio::AudioHandle, model::track::Track};
use iced::{
    Alignment::Center,
    Element, Length, Subscription, Task,
    keyboard::Key,
    widget::{Column, Container, Image, image, row, text},
};

const ALBUM_ART_PLACEHOLDER: &[u8] = include_bytes!("album_art.png");
static ALBUM_ART_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| image::Handle::from_bytes(ALBUM_ART_PLACEHOLDER));

#[derive(Debug, PartialEq, Default, Clone)]
enum PlayerState {
    #[default]
    Paused,
    Playing,
}

#[derive(Debug, Clone)]
pub enum PlayerMessage {
    Play,
    Pause,
    Playing,
    Load(Track),
    Input(Key),
    Error(String),
}

pub struct Player {
    state: PlayerState,
    track: Option<Track>,
    audio: Option<AudioHandle>,
    position: Option<Duration>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            state: PlayerState::Paused,
            track: None,
            audio: None,
            position: None,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            state: PlayerState::default(),
            track: None,
            audio: None,
            position: None,
        }
    }

    pub fn view(&self) -> Element<PlayerMessage> {
        let album_art = Image::new(ALBUM_ART_HANDLE.clone())
            .width(Length::Fixed(80.0))
            .height(Length::Fixed(80.0));

        let content = match &self.track {
            Some(track) => Column::new()
                .align_x(Center)
                .push(album_art)
                .push(text(track.title.clone()))
                .push(text(track.artist.clone().unwrap_or_default()))
                .push(row![
                    text(
                        self.position
                            .map(format_duration)
                            .unwrap_or_else(|| "--:--".into())
                    ),
                    text(" / "),
                    text(
                        self.track
                            .as_ref()
                            .map(|t| format_duration(t.duration))
                            .unwrap_or_else(|| "--:--".into())
                    ),
                ]),
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
            PlayerMessage::Playing => {
                if let Some(audio) = &self.audio {
                    self.position = Some(audio.position());
                }

                Task::none()
            }
            _ => todo!(),
        }
    }

    pub fn subscription(&self) -> Subscription<PlayerMessage> {
        if self.state == PlayerState::Playing {
            iced::time::every(Duration::from_millis(100)).map(|_| PlayerMessage::Playing)
        } else {
            Subscription::none()
        }
    }
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{:02}:{:02}", mins, secs)
}
