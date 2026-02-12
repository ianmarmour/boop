use iced::{
    Color, Element, Padding, Task, Theme, border,
    keyboard::{self, Key},
    widget::container,
    widget::container::Style,
};
use tracing::info;

use crate::{
    frontend::{
        library::view::{Library, LibraryMessage},
        player::{Player, PlayerMessage},
    },
    service::CatalogService,
};

#[derive(Default, Debug, Clone)]
pub enum ApplicationView {
    #[default]
    Library,
    Player,
}

#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    Library(LibraryMessage),
    Player(PlayerMessage),
    Input(Key),
}

pub struct Application {
    current_view: ApplicationView,
    pub player: Player,
    pub library: Library,
}

impl Application {
    pub fn new(catalog: CatalogService) -> (Application, Task<ApplicationMessage>) {
        let (library, library_task) = Library::new(catalog.clone());

        let application = Application {
            current_view: ApplicationView::default(),
            player: Player::default(),
            library: library,
        };

        (application, library_task.map(ApplicationMessage::Library))
    }

    pub fn view(&self) -> Element<ApplicationMessage> {
        let view_element = match self.current_view {
            ApplicationView::Library => self.library.view().map(ApplicationMessage::Library),
            ApplicationView::Player => self.player.view().map(ApplicationMessage::Player),
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
            ApplicationMessage::Library(message) => match message {
                LibraryMessage::TrackSelect(track) => {
                    self.current_view = ApplicationView::Player;
                    Task::done(ApplicationMessage::Player(PlayerMessage::Load(track)))
                }
                other => self.library.update(other).map(ApplicationMessage::Library),
            },
            ApplicationMessage::Input(key) => match self.current_view {
                ApplicationView::Library => self
                    .library
                    .update(LibraryMessage::InputEvent(key))
                    .map(ApplicationMessage::Library),
                ApplicationView::Player => match key {
                    Key::Named(keyboard::key::Named::Backspace) => {
                        self.current_view = ApplicationView::Library;
                        Task::none()
                    }
                    _ => Task::none(),
                },
            },
            ApplicationMessage::Player(message) => {
                self.player.update(message).map(ApplicationMessage::Player)
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<ApplicationMessage> {
        iced::Subscription::batch([
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
                    } => Some(ApplicationMessage::Input(key)),
                    _ => None,
                }
            }),
            self.player.subscription().map(ApplicationMessage::Player),
        ])
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
