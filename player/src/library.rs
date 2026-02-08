use std::{fmt::Debug, slice};

use iced::{
    Background, Color, Element, Length, Padding, Task, Theme,
    keyboard::Key,
    widget::{
        button,
        button::{Status, Style},
        keyed::Column,
        text,
    },
};
use model::{CatalogItem, artist::Artist, release::Release, track::Track};
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct LibraryItem {
    pub id: u64,
    pub selected: bool,
    pub catalog_item: CatalogItem,
}

impl LibraryItem {
    pub fn new(catalog_data: CatalogItem) -> Self {
        match catalog_data {
            CatalogItem::Artist(data) => Self {
                id: data.id,
                selected: false,
                catalog_item: CatalogItem::Artist(data),
            },
            CatalogItem::Release(data) => Self {
                id: data.id,
                selected: false,
                catalog_item: CatalogItem::Release(data),
            },
            CatalogItem::Track(data) => Self {
                id: data.id,
                selected: false,
                catalog_item: CatalogItem::Track(data),
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
            CatalogItem::Artist(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::ItemExpand(self.id))
                .width(Length::Fill)
                .style(button_style)
                .into(),
            CatalogItem::Release(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::ItemExpand(self.id))
                .width(Length::Fill)
                .style(button_style)
                .into(),
            CatalogItem::Track(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::TrackSelect(data.clone()))
                .width(Length::Fill)
                .style(button_style)
                .into(),
        }
    }

    pub fn is_expandable(&self) -> bool {
        match self.catalog_item {
            CatalogItem::Artist(_) => true,
            CatalogItem::Release(_) => true,
            CatalogItem::Track(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LibraryMessage {
    TrackSelect(Track),
    ItemExpand(u64),
    ItemsLoad(u64, Vec<LibraryItem>),
    InputEvent(Key),
}

#[derive(Debug, Clone)]
pub enum LibraryItemAction {
    SelectNext,
    SelectPrevious,
}

#[derive(Debug, Clone)]
pub struct LibraryItems {
    cursor: Option<usize>,
    inner: Vec<LibraryItem>,
}

impl LibraryItems {
    pub fn new(items: Vec<LibraryItem>) -> Self {
        Self {
            cursor: None,
            inner: items,
        }
    }

    pub fn select(&mut self, action: LibraryItemAction) {
        debug!("item select action invoked: {:?}", action);

        if let Some(cursor) = self.cursor {
            self.inner[cursor].toggled_selected();
        }

        match action {
            LibraryItemAction::SelectNext => match self.cursor {
                Some(index) => {
                    if index < self.inner.len() - 1 {
                        self.cursor = Some(index + 1)
                    }
                }
                None => self.cursor = Some(0),
            },
            LibraryItemAction::SelectPrevious => match self.cursor {
                Some(index) => match index {
                    0 => self.cursor = Some(index),
                    _ => self.cursor = Some(index - 1),
                },
                None => self.cursor = None, // TODO: Could throw an error using result???
            },
        }

        if let Some(cursor) = self.cursor {
            self.inner[cursor].toggled_selected();
        }
    }

    pub fn selected(&self) -> Option<&LibraryItem> {
        match self.cursor {
            Some(index) => self.inner.get(index),
            None => None,
        }
    }

    pub fn refresh(&mut self, items: Vec<LibraryItem>) -> &Self {
        self.inner = items;
        self.cursor = None;

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
            catalog_item: CatalogItem::Artist(Artist {
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
            LibraryMessage::ItemExpand(id) => {
                let expanded_item = self.items.iter().find(|item| item.id == id);

                match expanded_item {
                    Some(item) => self.load_children(&item),
                    None => todo!(),
                }
            }
            LibraryMessage::ItemsLoad(_, items) => {
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
                            CatalogItem::Track(t) => {
                                Task::done(LibraryMessage::TrackSelect(t.clone()))
                            }
                            _ => self.load_children(&item),
                        },
                        None => todo!(),
                    },
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

    // Library handles all loading logic
    fn load_children(&self, item: &LibraryItem) -> Task<LibraryMessage> {
        let id = item.id;

        match &item.catalog_item {
            CatalogItem::Artist(artist) => {
                Task::perform(load_releases(artist.releases.clone()), move |items| {
                    LibraryMessage::ItemsLoad(
                        id,
                        items
                            .iter()
                            .map(|release| {
                                LibraryItem::new(CatalogItem::Release(release.to_owned()))
                            })
                            .collect(),
                    )
                })
            }
            CatalogItem::Release(release) => {
                Task::perform(load_tracks(release.tracks.clone()), move |items| {
                    LibraryMessage::ItemsLoad(
                        id,
                        items
                            .iter()
                            .map(|track| LibraryItem::new(CatalogItem::Track(track.to_owned())))
                            .collect(),
                    )
                })
            }
            CatalogItem::Track(track) => Task::none(),
            _ => Task::none(),
        }
    }
}

async fn load_releases(ids: Vec<u64>) -> Vec<Release> {
    ids.iter()
        .map(|id| Release {
            id: id.clone(),
            name: format!("test-release-{}", id).to_string(),
            tracks: vec![0, 1, 2, 3],
            artist: 0,
        })
        .collect()
}

async fn load_tracks(ids: Vec<u64>) -> Vec<Track> {
    ids.iter()
        .map(|id| Track {
            id: id.clone(),
            name: format!("test-track-{}", id).to_string(),
            release: 0,
            artist: 0,
        })
        .collect()
}
