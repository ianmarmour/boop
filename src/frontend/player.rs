use std::{fmt::Debug, time::Duration};

use crate::{audio::AudioHandle, model::track::Track};
use iced::{
    Alignment::Center,
    Element, Length, Subscription, Task,
    advanced::image::Handle as ImageHandle,
    keyboard::{Key, key::Named},
    time::every,
    widget::{Column, Container, Image, row, text},
};
use tracing::debug;

#[derive(Debug, PartialEq, Default, Clone)]
pub enum PlayerState {
    #[default]
    Paused,
    Playing,
}

impl std::fmt::Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerState::Playing => write!(f, "PLAY"),
            PlayerState::Paused => write!(f, "PAUSE"),
        }
    }
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
    cover: Option<ImageHandle>,
    position: Option<Duration>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            state: PlayerState::Paused,
            track: None,
            audio: None,
            cover: None,
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
            cover: None,
            position: None,
        }
    }

    pub fn view(&self) -> Element<'_, PlayerMessage> {
        let content = match &self.track {
            Some(track) => match &self.cover {
                Some(cover) => Column::new()
                    .align_x(Center)
                    .push(
                        Container::new(Image::new(cover).width(Length::Fill).height(Length::Fill))
                            .padding(40),
                    )
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
                None => Column::new(),
            },
            None => Column::new().align_x(Center).push(text("No track loaded")),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: PlayerMessage) -> Task<PlayerMessage> {
        match message {
            PlayerMessage::Load(track) => {
                if let Some(inner) = &self.track {
                    if &track == inner {
                        return Task::none();
                    }
                }

                let handle = AudioHandle::new();
                handle.load(&track.path);

                self.track = Some(track.clone());
                self.audio = Some(handle);

                self.cover = Some(ImageHandle::from_bytes(track.cover().byte_data));

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
            PlayerMessage::Input(key) => match key.as_ref() {
                Key::Named(Named::Space) => match self.state {
                    PlayerState::Playing => Task::done(PlayerMessage::Pause),
                    PlayerState::Paused => Task::done(PlayerMessage::Play),
                },
                _ => Task::none(),
            },
            _ => todo!(),
        }
    }

    pub fn subscription(&self) -> Subscription<PlayerMessage> {
        if self.state == PlayerState::Playing {
            debug!("playback event emitting");
            every(Duration::from_millis(100)).map(|_| PlayerMessage::Playing)
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
