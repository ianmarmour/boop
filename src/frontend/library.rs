use std::{fmt::Debug, slice};

use crate::model::{CatalogItem, CatalogItemKind, artist::Artist, release::Release, track::Track};
use iced::{
    Background, Color, Element, Length, Task, Theme,
    futures::FutureExt,
    keyboard::Key,
    widget::{
        button,
        button::{Status, Style},
        keyed::Column,
        text,
    },
};
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub struct LibraryItem {
    pub id: u64,
    pub selected: bool,
    pub catalog_item: CatalogItemKind,
}

impl LibraryItem {
    pub fn new(catalog_data: CatalogItemKind) -> Self {
        match catalog_data {
            CatalogItemKind::Artist(data) => Self {
                id: data.id,
                selected: false,
                catalog_item: CatalogItemKind::Artist(data),
            },
            CatalogItemKind::Release(data) => Self {
                id: data.id,
                selected: false,
                catalog_item: CatalogItemKind::Release(data),
            },
            CatalogItemKind::Track(data) => Self {
                id: data.id,
                selected: false,
                catalog_item: CatalogItemKind::Track(data),
            },
        }
    }

    pub fn toggled_selected(&mut self) {
        self.selected = !self.selected
    }

    pub fn view(&self) -> Element<LibraryMessage> {
        let button_style = |theme: &Theme, status: Status| -> Style {
            let palette = theme.extended_palette();

            let background = match self.selected {
                true => Some(Background::Color(Color::BLACK)),
                false => Some(Background::Color(Color::TRANSPARENT)),
            };

            Style {
                background: background,
                ..Style::default()
            }
        };

        match &self.catalog_item {
            CatalogItemKind::Artist(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::ItemLoad(Some(LibraryItem::new(
                    CatalogItemKind::Artist(data.clone()),
                ))))
                .width(Length::Fill)
                .style(button_style)
                .into(),
            CatalogItemKind::Release(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::ItemLoad(Some(LibraryItem::new(
                    CatalogItemKind::Release(data.clone()),
                ))))
                .width(Length::Fill)
                .style(button_style)
                .into(),
            CatalogItemKind::Track(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::TrackSelect(data.clone()))
                .width(Length::Fill)
                .style(button_style)
                .into(),
        }
    }

    pub fn is_expandable(&self) -> bool {
        match self.catalog_item {
            CatalogItemKind::Artist(_) => true,
            CatalogItemKind::Release(_) => true,
            CatalogItemKind::Track(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LibraryMessage {
    TrackSelect(Track),
    ItemRefresh(Vec<CatalogItemKind>),
    ItemLoad(Option<LibraryItem>),
    InputEvent(Key),
}

#[derive(Debug, Clone)]
pub enum LibraryItemAction {
    SelectNext,
    SelectPrevious,
}

#[derive(Debug, Clone)]
pub struct LibraryItems {
    inner: Vec<LibraryItem>,
}

impl LibraryItems {
    pub fn new(items: Vec<LibraryItem>) -> Self {
        Self { inner: items }
    }

    pub fn find(&self, catalog_id: u64) -> Option<&LibraryItem> {
        self.inner
            .iter()
            .find(|item| item.catalog_item.clone().id() == catalog_id)
    }

    pub fn select(&mut self, action: LibraryItemAction) {
        debug!("item select action invoked: {:?}", action);

        let selected_idx = self.inner.iter().position(|item| item.selected == true);

        match selected_idx {
            Some(index) => {
                // Toggled the element as no longer selected
                self.inner[index].toggled_selected();

                match action {
                    LibraryItemAction::SelectNext => {
                        if index < self.inner.len() - 1 {
                            self.inner[index + 1].toggled_selected();
                        } else {
                            self.inner[index].toggled_selected();
                        }
                    }
                    LibraryItemAction::SelectPrevious => {
                        if index > 0 {
                            self.inner[index - 1].toggled_selected();
                        } else {
                            self.inner[index].toggled_selected();
                        }
                    }
                }
            }
            None => self.inner[0].toggled_selected(),
        }
    }

    pub fn selected(&self) -> Option<&LibraryItem> {
        info!("items: {:?}", self.inner);

        self.inner.iter().find(|item| item.selected == true)
    }

    pub fn refresh(&mut self, items: Vec<LibraryItem>) -> &Self {
        self.inner = items;
        self.inner[0].toggled_selected();

        self
    }

    pub fn iter(&self) -> slice::Iter<'_, LibraryItem> {
        self.inner.iter()
    }
}

impl IntoIterator for LibraryItems {
    type Item = LibraryItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a LibraryItems {
    type Item = &'a LibraryItem;
    type IntoIter = slice::Iter<'a, LibraryItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter() // Reuses the conventional iter method
    }
}

#[derive(Debug, Clone)]
pub struct Library {
    items: LibraryItems,
}

impl Default for Library {
    fn default() -> Self {
        let items = vec![LibraryItem {
            id: 012345,
            catalog_item: CatalogItemKind::Artist(Artist {
                id: 012345,
                name: "test-artist".to_string(),
                releases: vec![0, 1, 2, 3, 4, 5],
            }),
            selected: false,
        }];

        Library {
            items: LibraryItems::new(items),
        }
    }
}

impl Library {
    pub fn new(items: Vec<LibraryItem>) -> Library {
        Library {
            items: LibraryItems::new(items),
        }
    }

    pub fn update(&mut self, message: LibraryMessage) -> Task<LibraryMessage> {
        match message {
            LibraryMessage::ItemLoad(item) => match item {
                Some(library_item) => match library_item.catalog_item {
                    CatalogItemKind::Artist(a) => {
                        Task::perform(list_releases(a.id), LibraryMessage::ItemRefresh)
                    }
                    CatalogItemKind::Release(r) => {
                        Task::perform(list_tracks(r.id), LibraryMessage::ItemRefresh)
                    }
                    _ => Task::none(),
                },
                None => Task::perform(list_artists(), LibraryMessage::ItemRefresh),
            },
            LibraryMessage::ItemRefresh(items) => {
                error!("We're cooked boys: {:?}", items);
                let items = items
                    .iter()
                    .map(|item| LibraryItem::new(item.clone()))
                    .collect();
                error!("We're cooked boys: {:?}", items);

                self.items.refresh(items);

                Task::none()
            }
            LibraryMessage::InputEvent(key) => {
                info!("input key event detected: {:?}", key);

                match key.as_ref() {
                    Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                        self.items.select(LibraryItemAction::SelectPrevious);
                        Task::none()
                    }
                    Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                        self.items.select(LibraryItemAction::SelectNext);
                        Task::none()
                    }
                    Key::Named(iced::keyboard::key::Named::Enter) => match self.items.selected() {
                        Some(item) => match &item.catalog_item {
                            CatalogItemKind::Track(t) => {
                                Task::done(LibraryMessage::TrackSelect(t.clone()))
                            }
                            CatalogItemKind::Artist(a) => Task::perform(
                                get_artist(a.id).map(LibraryItem::new).map(Some),
                                LibraryMessage::ItemLoad,
                            ),
                            CatalogItemKind::Release(r) => Task::perform(
                                get_release(r.id).map(LibraryItem::new).map(Some),
                                LibraryMessage::ItemLoad,
                            ),
                        },
                        None => todo!(),
                    },
                    Key::Named(iced::keyboard::key::Named::Backspace) => {
                        match self.items.selected() {
                            Some(item) => match &item.catalog_item {
                                CatalogItemKind::Release(r) => {
                                    error!("fetching releases");
                                    Task::done(LibraryMessage::ItemLoad(None))
                                }
                                CatalogItemKind::Track(t) => Task::perform(
                                    get_artist(t.artist).map(LibraryItem::new).map(Some),
                                    LibraryMessage::ItemLoad,
                                ),
                                _ => Task::none(),
                            },
                            None => todo!(),
                        }
                    }
                    _ => Task::none(),
                }
            }
            LibraryMessage::TrackSelect(track) => {
                todo!()
            }
        }
    }

    pub fn view(&self) -> Element<LibraryMessage> {
        let mut col = Column::new().spacing(5);

        // Render all artists
        for item in self.items.iter() {
            col = col.push(item.id, item.view());
        }

        col.into()
    }
}

async fn get_artist(id: u64) -> CatalogItemKind {
    CatalogItemKind::Artist(Artist {
        id: id.clone(),
        name: format!("test-release-{}", id).to_string(),
        releases: vec![0, 1, 2, 3],
    })
}

async fn list_artists() -> Vec<CatalogItemKind> {
    vec![CatalogItemKind::Artist(Artist {
        id: 0,
        name: format!("test-artist-{}", 0).to_string(),
        releases: vec![0, 1, 2],
    })]
}

async fn get_release(id: u64) -> CatalogItemKind {
    CatalogItemKind::Release(Release {
        id: id.clone(),
        name: format!("test-release-{}", id).to_string(),
        tracks: vec![0, 1, 2, 3],
        artist: 0,
    })
}

async fn list_releases(artist_id: u64) -> Vec<CatalogItemKind> {
    vec![
        CatalogItemKind::Release(Release {
            id: 0,
            name: format!("test-release-{}", 0).to_string(),
            tracks: vec![0, 1, 2],
            artist: artist_id,
        }),
        CatalogItemKind::Release(Release {
            id: 1,
            name: format!("test-release-{}", 1).to_string(),
            tracks: vec![3, 4, 5],
            artist: artist_id,
        }),
        CatalogItemKind::Release(Release {
            id: 2,
            name: format!("test-release-{}", 2).to_string(),
            tracks: vec![6, 7, 8],
            artist: artist_id,
        }),
    ]
}

async fn get_track(id: u64) -> CatalogItemKind {
    CatalogItemKind::Track(Track {
        id: id.clone(),
        name: format!("test-track-{}", id).to_string(),
        release: 0,
        artist: 0,
    })
}

async fn list_tracks(release_id: u64) -> Vec<CatalogItemKind> {
    vec![
        CatalogItemKind::Track(Track {
            id: 0,
            name: format!("test-track-{}", 0).to_string(),
            release: release_id,
            artist: 0,
        }),
        CatalogItemKind::Track(Track {
            id: 1,
            name: format!("test-track-{}", 1).to_string(),
            release: release_id,
            artist: 0,
        }),
        CatalogItemKind::Track(Track {
            id: 2,
            name: format!("test-track-{}", 2).to_string(),
            release: release_id,
            artist: 0,
        }),
    ]
}
