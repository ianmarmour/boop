use std::{fmt::Debug, slice};

use crate::{
    model::{CatalogItem, CatalogMetadata, artist::Artist, release::Release, track::Track},
    repository::{artist::ArtistFilter, release::ReleaseFilter, track::TrackFilter},
    service::CatalogService,
};
use iced::{
    Background, Color, Element, Length, Task, Theme,
    keyboard::Key,
    widget::{
        button,
        button::{Status, Style},
        keyed::Column,
        text,
    },
};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Clone)]
pub struct LibraryItem {
    pub catalog_item: CatalogItem<CatalogMetadata>,
    pub selected: bool,
}

impl LibraryItem {
    pub fn new(catalog_item: CatalogItem<CatalogMetadata>) -> Self {
        Self {
            catalog_item,
            selected: false,
        }
    }

    pub fn view(&self) -> Element<'_, LibraryMessage> {
        let button_style = |theme: &Theme, _: Status| -> Style {
            let palette = theme.extended_palette();

            let background = match self.selected {
                true => Some(Background::Color(palette.background.neutral.color)),
                false => Some(Background::Color(Color::TRANSPARENT)),
            };

            Style {
                background: background,
                ..Style::default()
            }
        };

        button(text(self.catalog_item.metadata.display_text()))
            .on_press(LibraryMessage::ItemLoad(Some(LibraryItem::new(
                self.catalog_item.clone(),
            ))))
            .width(Length::Fill)
            .style(button_style)
            .into()
    }

    pub fn toggled_selected(&mut self) {
        self.selected = !self.selected
    }

    pub fn is_expandable(&self) -> bool {
        match self.catalog_item.metadata {
            CatalogMetadata::Artist(_) => true,
            CatalogMetadata::Release(_) => true,
            CatalogMetadata::Track(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LibraryMessage {
    ItemsLoad(Vec<LibraryItem>),
    TrackSelect(Track),
    ItemRefresh(Vec<CatalogItem<CatalogMetadata>>),
    ItemLoad(Option<LibraryItem>),
    InputEvent(Key),
    Error(String),
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

    pub fn find(&self, catalog_id: i64) -> Option<&LibraryItem> {
        self.inner
            .iter()
            .find(|item| item.catalog_item.clone().id == catalog_id)
    }

    pub fn select(&mut self, action: LibraryItemAction) {
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
        self.iter()
    }
}

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("unknown error occured")]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Library {
    catalog: Option<CatalogService>,
    items: LibraryItems,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            catalog: None,
            items: LibraryItems::new(vec![]),
        }
    }
}

impl Library {
    pub fn new(catalog: CatalogService) -> (Library, Task<LibraryMessage>) {
        let library = Self {
            catalog: Some(catalog.clone()),
            items: LibraryItems::new(vec![]),
        };

        let task = Task::perform(
            async move {
                catalog
                    .artist
                    .lock()
                    .await
                    .list_artists(ArtistFilter::default())
                    .await
            },
            |result| match result {
                Ok(items) => LibraryMessage::ItemsLoad(
                    items
                        .into_iter()
                        .map(|i| LibraryItem::new(i.into()))
                        .collect(),
                ),
                Err(e) => LibraryMessage::Error(e.to_string()),
            },
        );

        (library, task)
    }

    pub fn view(&self) -> Element<'_, LibraryMessage> {
        let mut col = Column::new().spacing(5);

        for item in self.items.iter() {
            col = col.push(item.catalog_item.id, item.view());
        }

        col.into()
    }

    pub fn update(&mut self, message: LibraryMessage) -> Task<LibraryMessage> {
        let Some(catalog) = self.catalog.clone() else {
            return Task::done(LibraryMessage::Error("No catalog".into()));
        };

        match message {
            LibraryMessage::ItemsLoad(items) => {
                self.items = LibraryItems::new(items);
                Task::none()
            }
            LibraryMessage::ItemLoad(item) => match item {
                Some(library_item) => match library_item.catalog_item.metadata {
                    CatalogMetadata::Artist(a) => {
                        let artist = a.clone();

                        Task::perform(
                            async move {
                                let releases = catalog
                                    .release
                                    .lock()
                                    .await
                                    .list_releases(ReleaseFilter {
                                        title: None,
                                        artist: Some(artist.name),
                                    })
                                    .await
                                    .map_err(|_| LibraryError::Unknown)?;

                                Ok::<Vec<CatalogItem<CatalogMetadata>>, LibraryError>(
                                    releases.into_iter().map(Into::into).collect(),
                                )
                            },
                            |result| match result {
                                Ok(items) => LibraryMessage::ItemRefresh(items),
                                Err(e) => LibraryMessage::Error(e.to_string()),
                            },
                        )
                    }
                    CatalogMetadata::Release(r) => {
                        let release = r.clone();

                        Task::perform(
                            async move {
                                let tracks = catalog
                                    .track
                                    .lock()
                                    .await
                                    .list_tracks(TrackFilter {
                                        name: None,
                                        artist: release.artist,
                                        release: Some(release.title),
                                    })
                                    .await
                                    .map_err(|e| LibraryError::Internal(e.into()))?;

                                Ok::<Vec<CatalogItem<CatalogMetadata>>, LibraryError>(
                                    tracks.into_iter().map(Into::into).collect(),
                                )
                            },
                            |result| match result {
                                Ok(items) => LibraryMessage::ItemRefresh(items),
                                Err(e) => LibraryMessage::Error(e.to_string()),
                            },
                        )
                    }
                    _ => Task::none(),
                },
                None => Task::perform(
                    async move {
                        let releases = catalog
                            .artist
                            .lock()
                            .await
                            .list_artists(ArtistFilter::default())
                            .await
                            .map_err(|e| LibraryError::Internal(e.into()))?;

                        Ok::<Vec<CatalogItem<CatalogMetadata>>, LibraryError>(
                            releases.into_iter().map(Into::into).collect(),
                        )
                    },
                    |result| match result {
                        Ok(items) => LibraryMessage::ItemRefresh(items),
                        Err(e) => LibraryMessage::Error(e.to_string()),
                    },
                ),
            },
            LibraryMessage::ItemRefresh(items) => {
                let items = items
                    .iter()
                    .map(|item| LibraryItem::new(item.clone()))
                    .collect();

                self.items.refresh(items);

                Task::none()
            }
            LibraryMessage::InputEvent(key) => match key.as_ref() {
                Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                    self.items.select(LibraryItemAction::SelectPrevious);
                    Task::none()
                }
                Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                    self.items.select(LibraryItemAction::SelectNext);
                    Task::none()
                }
                Key::Named(iced::keyboard::key::Named::Enter) => match self.items.selected() {
                    Some(item) => match &item.catalog_item.metadata {
                        CatalogMetadata::Track(t) => {
                            Task::done(LibraryMessage::TrackSelect(t.clone()))
                        }
                        CatalogMetadata::Artist(a) => {
                            let name = a.name.clone();
                            Task::perform(
                                async move {
                                    let item = catalog
                                        .artist
                                        .lock()
                                        .await
                                        .get_artist(&name)
                                        .await
                                        .map_err(|e| LibraryError::Internal(e.into()))?;
                                    Ok::<CatalogItem<CatalogMetadata>, LibraryError>(item.into())
                                },
                                |result| match result {
                                    Ok(item) => {
                                        LibraryMessage::ItemLoad(Some(LibraryItem::new(item)))
                                    }
                                    Err(e) => LibraryMessage::Error(e.to_string()),
                                },
                            )
                        }
                        CatalogMetadata::Release(r) => {
                            let title = r.title.clone();
                            Task::perform(
                                async move {
                                    let item = catalog
                                        .release
                                        .lock()
                                        .await
                                        .get_release(&title)
                                        .await
                                        .map_err(|e| LibraryError::Internal(e.into()))?;
                                    Ok::<CatalogItem<CatalogMetadata>, LibraryError>(item.into())
                                },
                                |result| match result {
                                    Ok(item) => {
                                        LibraryMessage::ItemLoad(Some(LibraryItem::new(item)))
                                    }
                                    Err(e) => LibraryMessage::Error(e.to_string()),
                                },
                            )
                        }
                    },
                    None => todo!(),
                },
                Key::Named(iced::keyboard::key::Named::Backspace) => match self.items.selected() {
                    Some(item) => match &item.catalog_item.metadata {
                        CatalogMetadata::Release(_) => Task::perform(
                            async move {
                                let items = catalog
                                    .artist
                                    .lock()
                                    .await
                                    .list_artists(ArtistFilter {
                                        name: None,
                                        track: None,
                                    })
                                    .await
                                    .map_err(|e| LibraryError::Internal(e.into()))?;

                                Ok::<Vec<CatalogItem<Artist>>, LibraryError>(items)
                            },
                            |result| match result {
                                Ok(item) => LibraryMessage::ItemsLoad(
                                    item.iter()
                                        .map(|i| LibraryItem::new(i.to_owned().into()))
                                        .collect(),
                                ),
                                Err(e) => LibraryMessage::Error(e.to_string()),
                            },
                        ),
                        CatalogMetadata::Track(t) => {
                            let track = t.clone();
                            Task::perform(
                                async move {
                                    let items = catalog
                                        .release
                                        .lock()
                                        .await
                                        .list_releases(ReleaseFilter {
                                            title: None,
                                            artist: track.artist,
                                        })
                                        .await
                                        .map_err(|_| LibraryError::Unknown)?;

                                    Ok::<Vec<CatalogItem<Release>>, LibraryError>(items)
                                },
                                |result| match result {
                                    Ok(item) => LibraryMessage::ItemsLoad(
                                        item.iter()
                                            .map(|i| LibraryItem::new(i.to_owned().into()))
                                            .collect(),
                                    ),
                                    Err(e) => LibraryMessage::Error(e.to_string()),
                                },
                            )
                        }
                        _ => Task::none(),
                    },
                    None => {
                        self.items.select(LibraryItemAction::SelectNext);
                        Task::none()
                    }
                },
                _ => Task::none(),
            },
            LibraryMessage::TrackSelect(track) => {
                todo!()
            }
            LibraryMessage::Error(test) => {
                info!(test);
                Task::none()
                //todo!()
            }
        }
    }
}
