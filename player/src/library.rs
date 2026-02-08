use std::fmt::Debug;

use iced::{
    Element, Length, Padding, Task, Theme,
    widget::{button, keyed::Column, text},
};
use model::{CatalogItem, artist::Artist, release::Release, track::Track};

#[derive(Debug, Clone)]
pub struct LibraryItem {
    pub id: u64,
    pub catalog_item: CatalogItem,
}

impl LibraryItem {
    pub fn new(catalog_data: CatalogItem) -> Self {
        match catalog_data {
            CatalogItem::Artist(data) => Self {
                id: data.id,
                catalog_item: CatalogItem::Artist(data),
            },
            CatalogItem::Release(data) => Self {
                id: data.id,
                catalog_item: CatalogItem::Release(data),
            },
            CatalogItem::Track(data) => Self {
                id: data.id,
                catalog_item: CatalogItem::Track(data),
            },
        }
    }

    pub fn view(&self) -> Element<LibraryMessage> {
        match &self.catalog_item {
            CatalogItem::Artist(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::ItemExpand(self.id))
                .width(Length::Fill)
                .into(),
            CatalogItem::Release(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::ItemExpand(self.id))
                .width(Length::Fill)
                .into(),
            CatalogItem::Track(data) => button(text(data.name.clone()))
                .on_press(LibraryMessage::TrackSelect(data.clone()))
                .width(Length::Fill)
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
}

#[derive(Debug, Clone)]
pub struct Library {
    items: Vec<LibraryItem>,
}

impl Default for Library {
    fn default() -> Self {
        Library {
            items: vec![LibraryItem {
                id: 012345,
                catalog_item: CatalogItem::Artist(Artist {
                    id: 012345,
                    name: "test-artist".to_string(),
                    releases: vec![0, 1, 2, 3, 4, 5],
                }),
            }],
        }
    }
}

impl Library {
    pub fn new(items: Vec<LibraryItem>) -> Library {
        Library { items }
    }

    pub fn update(&mut self, message: LibraryMessage) -> Task<LibraryMessage> {
        match message {
            LibraryMessage::ItemExpand(id) => {
                let expanded_item = self.items.iter().find(|item| item.id == id);

                match expanded_item {
                    Some(item) => self.load_children(item),
                    None => todo!(),
                }
            }
            LibraryMessage::ItemsLoad(_, items) => {
                self.items = items;
                Task::none()
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn view(&self) -> Element<LibraryMessage> {
        let mut col = Column::new().spacing(5);

        // Render all artists
        for item in &self.items {
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
        })
        .collect()
}

async fn load_tracks(ids: Vec<u64>) -> Vec<Track> {
    ids.iter()
        .map(|id| Track {
            id: id.clone(),
            name: format!("test-track-{}", id).to_string(),
        })
        .collect()
}
