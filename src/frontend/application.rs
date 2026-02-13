use iced::{
    Element, Padding, Subscription, Task,
    keyboard::{self, Key},
    widget::column,
    widget::container,
};
use tracing::debug;

use crate::{
    frontend::{
        library::{Library, LibraryMessage},
        menu::{Menu, MenuMessage},
        player::{Player, PlayerMessage, PlayerState},
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
    ChangeView(ApplicationView),
    Library(LibraryMessage),
    Menu(MenuMessage),
    Player(PlayerMessage),
    Input(Key),
}

pub struct Application {
    current_view: ApplicationView,
    pub player: Player,
    pub library: Library,
    pub menu: Menu,
}

impl Application {
    pub fn new(catalog: CatalogService) -> (Self, Task<ApplicationMessage>) {
        let (library, library_task) = Library::new(catalog.clone());

        let application = Self {
            current_view: ApplicationView::default(),
            player: Player::default(),
            library: library,
            menu: Menu::default(),
        };

        (application, library_task.map(ApplicationMessage::Library))
    }

    pub fn view(&self) -> Element<'_, ApplicationMessage> {
        let element = match self.current_view {
            ApplicationView::Library => self.library.view().map(ApplicationMessage::Library),
            ApplicationView::Player => self.player.view().map(ApplicationMessage::Player),
        };

        column![
            self.menu.view().map(ApplicationMessage::Menu),
            container(element)
                .padding(Padding {
                    top: 0.0,
                    bottom: 0.0,
                    left: 1.0,
                    right: 1.0,
                })
                .center_x(480)
                .height(640)
                .width(480)
        ]
        .into()
    }

    pub fn update(&mut self, message: ApplicationMessage) -> Task<ApplicationMessage> {
        match message {
            ApplicationMessage::ChangeView(view) => {
                self.current_view = view;
                Task::none()
            }
            ApplicationMessage::Library(message) => match message {
                LibraryMessage::TrackSelect(track) => {
                    Task::done(ApplicationMessage::Player(PlayerMessage::Load(track)))
                }
                message => self
                    .library
                    .update(message)
                    .map(ApplicationMessage::Library),
            },
            ApplicationMessage::Menu(message) => match message {
                message => self.menu.update(message).map(ApplicationMessage::Menu),
            },
            ApplicationMessage::Player(message) => {
                let task = self
                    .player
                    .update(message.clone())
                    .map(ApplicationMessage::Player);

                match message {
                    PlayerMessage::Load(_) => task.chain(Task::done(
                        ApplicationMessage::ChangeView(ApplicationView::Player),
                    )),
                    PlayerMessage::Play => Task::batch([
                        task,
                        Task::done(ApplicationMessage::Menu(MenuMessage::PlayerStatus(
                            PlayerState::Playing,
                        ))),
                    ]),
                    PlayerMessage::Pause => Task::batch([
                        task,
                        Task::done(ApplicationMessage::Menu(MenuMessage::PlayerStatus(
                            PlayerState::Paused,
                        ))),
                    ]),
                    _ => task,
                }
            }
            ApplicationMessage::Input(key) => match self.current_view {
                ApplicationView::Library => self
                    .library
                    .update(LibraryMessage::InputEvent(key))
                    .map(ApplicationMessage::Library),
                ApplicationView::Player => match key {
                    Key::Named(keyboard::key::Named::Backspace) => {
                        Task::done(ApplicationMessage::ChangeView(ApplicationView::Library))
                    }
                    key => self
                        .player
                        .update(PlayerMessage::Input(key))
                        .map(ApplicationMessage::Player),
                },
            },
        }
    }

    pub fn subscription(&self) -> iced::Subscription<ApplicationMessage> {
        Subscription::batch([
            keyboard::listen().filter_map(|event| {
                debug!("keyboard event detected: {:?}", event);
                match event {
                    keyboard::Event::KeyPressed { key, .. } => Some(ApplicationMessage::Input(key)),
                    _ => None,
                }
            }),
            self.player.subscription().map(ApplicationMessage::Player),
            self.menu.subscription().map(ApplicationMessage::Menu),
        ])
    }
}
