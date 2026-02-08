use iced::{
    Color, Element, Padding, Task, Theme, border,
    keyboard::{self, Key},
    widget::container,
    widget::container::Style,
};
use tracing::info;

use crate::{
    library::{Library, LibraryMessage},
    now_playing::{NowPlaying, NowPlayingMessage},
};

#[derive(Default, Debug, Clone)]
pub enum ApplicationView {
    #[default]
    Library,
    NowPlaying,
}

#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    LibraryMessage(LibraryMessage),
    NowPlaying(NowPlayingMessage),
    KeyboardInput(Key),
}

#[derive(Debug, Default, Clone)]
pub struct Application {
    active_view: ApplicationView,
    pub now_playing: NowPlaying,
    pub library: Library,
}

impl Application {
    pub fn new() -> Application {
        Application {
            active_view: ApplicationView::default(),
            now_playing: NowPlaying::default(),
            library: Library::default(),
        }
    }

    pub fn view(&self) -> Element<ApplicationMessage> {
        let view_element = match self.active_view {
            ApplicationView::Library => self.library.view().map(ApplicationMessage::LibraryMessage),
            ApplicationView::NowPlaying => {
                self.now_playing.view().map(ApplicationMessage::NowPlaying)
            }
        };

        container(view_element)
            .padding(Padding {
                top: 0.0,
                bottom: 0.0,
                left: 1.0,
                right: 1.0,
            })
            .center_x(480)
            .height(640)
            .width(480)
            .style(borderd_container)
            .into()
    }

    pub fn update(&mut self, message: ApplicationMessage) -> Task<ApplicationMessage> {
        match message {
            ApplicationMessage::LibraryMessage(message) => match message {
                LibraryMessage::TrackSelect(track) => {
                    self.now_playing = NowPlaying::new(track);
                    self.active_view = ApplicationView::NowPlaying;
                    Task::none()
                }
                other => self
                    .library
                    .update(other)
                    .map(ApplicationMessage::LibraryMessage),
            },
            ApplicationMessage::KeyboardInput(key) => match self.active_view {
                ApplicationView::Library => self
                    .library
                    .update(LibraryMessage::InputEvent(key))
                    .map(ApplicationMessage::LibraryMessage),
                ApplicationView::NowPlaying => todo!(),
            },
            ApplicationMessage::NowPlaying(_) => todo!(),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<ApplicationMessage> {
        keyboard::listen().filter_map(|event| {
            info!("keyboard subscribed key event detected: {:?}", event);

            match event {
                keyboard::Event::KeyPressed {
                    key,
                    modified_key,
                    physical_key,
                    location,
                    modifiers,
                    text,
                    repeat,
                } => Some(ApplicationMessage::KeyboardInput(key)),
                _ => None,
            }
        })
    }
}

pub fn borderd_container(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.base.color.into()),
        text_color: Some(palette.background.weak.text),
        border: border::rounded(0.0)
            .width(1.0) // Missing width makes border invisible
            .color(Color::BLACK), // Ensure color is applied last to be sure
        ..Style::default()
    }
}
