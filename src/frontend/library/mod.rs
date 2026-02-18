use std::{
    fmt::{Debug, Display, Formatter},
    slice,
};

use crate::{
    model::{CatalogItem, CatalogMetadata, artist::Artist, release::Release, track::Track},
    repository::{artist::ArtistFilter, release::ReleaseFilter, track::TrackFilter},
    service::CatalogService,
};
use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding, Shadow, Task, Theme,
    alignment::{Horizontal, Vertical},
    keyboard::Key,
    widget::{
        Id, Space, button,
        button::{Status, Style},
        container,
        image::{Handle, Image},
        keyed::Column,
        scrollable,
        scrollable::AutoScroll,
        text,
    },
};
use iced::{
    alignment,
    widget::{image, operation::scroll_to, row},
};
use thiserror::Error;
use tracing::info;

const FAVORITE_ICON: &[u8] = include_bytes!("../../resources/favorite.png");

const ITEM_HEIGHT: f32 = 60.0;
const ITEM_SPACING: f32 = 5.0;
const ROW_STRIDE: f32 = ITEM_HEIGHT + ITEM_SPACING;
const MENU_HEIGHT: f32 = 50.0;
const VIEWPORT_HEIGHT: f32 = 720.0 - MENU_HEIGHT; // 670.0

#[derive(Debug, Clone, Default)]
pub enum LibraryView {
    #[default]
    Artist,
    Release,
    Track,
}

impl Display for LibraryView {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            LibraryView::Artist => write!(f, "Artist"),
            LibraryView::Release => write!(f, "Release"),
            LibraryView::Track => write!(f, "Track"),
        }
    }
}

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

            Style {
                text_color: palette.background.base.text,
                ..Style::default()
            }
        };

        let button_text = match self.selected {
            false => text(self.catalog_item.metadata.display_text()),
            true => text(format!("> {}", self.catalog_item.metadata.display_text())),
        }
        .wrapping(text::Wrapping::None);

        button(button_text)
            .on_press(LibraryMessage::ItemLoad(Some(LibraryItem::new(
                self.catalog_item.clone(),
            ))))
            .width(Length::Fill)
            .height(Length::Fixed(ITEM_HEIGHT))
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
    ChangeView(LibraryView),
    InputEvent(Key),
    Scrolled(f32),
    Error(String),
}

#[derive(Debug, Clone)]
pub enum LibraryItemAction {
    SelectNext,
    SelectPrevious,
    SelectByCatalogId(i64),
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
                    LibraryItemAction::SelectByCatalogId(id) => {
                        if let Some(item_idx) = self
                            .inner
                            .iter()
                            .position(|item| item.catalog_item.id == id)
                        {
                            self.inner[item_idx].toggled_selected();
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
    scroll_id: Id,
    current_scroll_y: f32,
    favorite_image: Handle,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            catalog: None,
            items: LibraryItems::new(vec![]),
            scroll_id: Id::unique(),
            current_scroll_y: 0.0,
            favorite_image: Handle::from_bytes(FAVORITE_ICON),
        }
    }
}

impl Library {
    pub fn new(catalog: CatalogService) -> (Library, Task<LibraryMessage>) {
        let library = Self {
            catalog: Some(catalog.clone()),
            items: LibraryItems::new(vec![]),
            scroll_id: Id::unique(),
            current_scroll_y: 0.0,
            favorite_image: Handle::from_bytes(FAVORITE_ICON),
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
                Ok(items) => {
                    info!("ItemsLoad received {} items", items.len());
                    LibraryMessage::ItemsLoad(
                        items
                            .into_iter()
                            .map(|i| LibraryItem::new(i.into()))
                            .collect(),
                    )
                }
                Err(e) => LibraryMessage::Error(e.to_string()),
            },
        );

        (library, task)
    }

    pub fn view(&self) -> Element<'_, LibraryMessage> {
        let mut col = Column::new()
            .spacing(ITEM_SPACING)
            .padding(Padding::new(0.0).bottom(ITEM_HEIGHT));

        for item in self.items.iter() {
            let mut row = row![item.view()].height(ITEM_HEIGHT);

            if item.catalog_item.favorite {
                row = row.push(
                    container(image(&self.favorite_image).width(24).height(24))
                        .align_y(Vertical::Center)
                        .align_x(Horizontal::Right)
                        .width(44)
                        .height(ITEM_HEIGHT),
                );
            }

            col = col.push(item.catalog_item.id, row);
        }

        scrollable(col)
            .id(self.scroll_id.clone())
            .on_scroll(|viewport| {
                let y = viewport.absolute_offset().y;
                let snapped = (y / ROW_STRIDE).round() * ROW_STRIDE;
                LibraryMessage::Scrolled(snapped)
            })
            .style(|theme, status| scrollable::Style {
                container: container::Style::default(),
                vertical_rail: scrollable::Rail {
                    background: None,
                    border: Border::default(),
                    scroller: scrollable::Scroller {
                        border: Border::default(),
                        background: Background::Color(Color::TRANSPARENT),
                    },
                },
                horizontal_rail: scrollable::Rail {
                    background: None,
                    border: Border::default(),
                    scroller: scrollable::Scroller {
                        border: Border::default(),
                        background: Background::Color(Color::TRANSPARENT),
                    },
                },
                gap: None,
                auto_scroll: scrollable::AutoScroll {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border::default(),
                    shadow: Shadow::default(),
                    icon: Color::WHITE,
                },
            })
            .height(Length::Fill)
            .into()
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
            LibraryMessage::Scrolled(y) => {
                self.current_scroll_y = y;
                scroll_to(
                    self.scroll_id.clone(),
                    scrollable::AbsoluteOffset { x: 0.0, y },
                )
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

                    let selected_idx = self
                        .items
                        .inner
                        .iter()
                        .position(|item| item.selected)
                        .unwrap_or(0);

                    let item_top = selected_idx as f32 * ROW_STRIDE;

                    if item_top < self.current_scroll_y {
                        self.current_scroll_y = item_top;
                        scroll_to(
                            self.scroll_id.clone(),
                            scrollable::AbsoluteOffset {
                                x: 0.0,
                                y: item_top,
                            },
                        )
                    } else {
                        Task::none()
                    }
                }
                Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                    self.items.select(LibraryItemAction::SelectNext);

                    let selected_idx = self
                        .items
                        .inner
                        .iter()
                        .position(|item| item.selected)
                        .unwrap_or(0);

                    let item_top = selected_idx as f32 * ROW_STRIDE;
                    let item_bottom = item_top + ITEM_HEIGHT;

                    if item_bottom > self.current_scroll_y + VIEWPORT_HEIGHT {
                        let new_scroll_y = item_top - VIEWPORT_HEIGHT + ITEM_HEIGHT;
                        self.current_scroll_y = new_scroll_y;
                        scroll_to(
                            self.scroll_id.clone(),
                            scrollable::AbsoluteOffset {
                                x: 0.0,
                                y: new_scroll_y,
                            },
                        )
                    } else {
                        Task::none()
                    }
                }
                Key::Named(iced::keyboard::key::Named::Enter) => match self.items.selected() {
                    Some(item) => match &item.catalog_item.metadata {
                        CatalogMetadata::Track(t) => {
                            Task::done(LibraryMessage::TrackSelect(t.clone()))
                        }
                        CatalogMetadata::Artist(a) => {
                            let name = a.name.clone();
                            Task::batch(vec![
                                Task::perform(
                                    async move {
                                        let item = catalog
                                            .artist
                                            .lock()
                                            .await
                                            .get_artist(&name)
                                            .await
                                            .map_err(|e| LibraryError::Internal(e.into()))?;
                                        Ok::<CatalogItem<CatalogMetadata>, LibraryError>(
                                            item.into(),
                                        )
                                    },
                                    |result| match result {
                                        Ok(item) => {
                                            LibraryMessage::ItemLoad(Some(LibraryItem::new(item)))
                                        }
                                        Err(e) => LibraryMessage::Error(e.to_string()),
                                    },
                                ),
                                Task::done(LibraryMessage::ChangeView(LibraryView::Release)),
                            ])
                        }
                        CatalogMetadata::Release(r) => {
                            let title = r.title.clone();
                            Task::batch(vec![
                                Task::perform(
                                    async move {
                                        let item = catalog
                                            .release
                                            .lock()
                                            .await
                                            .get_release(&title)
                                            .await
                                            .map_err(|e| LibraryError::Internal(e.into()))?;
                                        Ok::<CatalogItem<CatalogMetadata>, LibraryError>(
                                            item.into(),
                                        )
                                    },
                                    |result| match result {
                                        Ok(item) => {
                                            LibraryMessage::ItemLoad(Some(LibraryItem::new(item)))
                                        }
                                        Err(e) => LibraryMessage::Error(e.to_string()),
                                    },
                                ),
                                Task::done(LibraryMessage::ChangeView(LibraryView::Track)),
                            ])
                        }
                    },
                    None => todo!(),
                },
                Key::Named(iced::keyboard::key::Named::Backspace) => match self.items.selected() {
                    Some(item) => match &item.catalog_item.metadata {
                        CatalogMetadata::Release(_) => Task::batch(vec![
                            Task::perform(
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
                            Task::done(LibraryMessage::ChangeView(LibraryView::Artist)),
                        ]),
                        CatalogMetadata::Track(t) => {
                            let track = t.clone();
                            Task::batch(vec![
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
                                ),
                                Task::done(LibraryMessage::ChangeView(LibraryView::Release)),
                            ])
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
            LibraryMessage::TrackSelect(track) => Task::none(),
            LibraryMessage::Error(test) => {
                info!(test);
                Task::none()
            }
            LibraryMessage::ChangeView(_) => Task::none(),
        }
    }
}
