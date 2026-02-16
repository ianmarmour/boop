use std::time::Duration;

use chrono::{DateTime, Local, Utc};
use iced::{
    Element, Length, Padding, Subscription, Task,
    alignment::Horizontal,
    time::every,
    widget::{Column, row, text},
};
use tracing::debug;

use crate::frontend::{application::ApplicationView, player::PlayerState};

#[derive(Debug, Clone)]
pub enum MenuMessage {
    ViewChange(ApplicationView),
    ClockTick,
}

#[derive(Debug, Clone)]
pub struct Menu {
    current_view: ApplicationView,
    datetime: DateTime<Local>,
    player_status: Option<PlayerState>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            current_view: ApplicationView::default(),
            datetime: Local::now(),
            player_status: None,
        }
    }
}

impl Menu {
    pub fn new(current_view: ApplicationView) -> Self {
        Self {
            current_view,
            datetime: Local::now(),
            player_status: None,
        }
    }
    pub fn view(&self) -> Element<'_, MenuMessage> {
        row![
            Column::new()
                .push(text(self.current_view.to_string()))
                .width(Length::FillPortion(1)),
            Column::new()
                .push(text(self.datetime.format("%H:%M").to_string()))
                .width(Length::Shrink)
                .align_x(Horizontal::Center),
            Column::new().width(Length::FillPortion(1)),
        ]
        .padding(Padding::new(0.0).horizontal(10.0))
        .into()
    }
    pub fn update(&mut self, message: MenuMessage) -> Task<MenuMessage> {
        match message {
            MenuMessage::ViewChange(view) => {
                self.current_view = view;
                Task::none()
            }
            MenuMessage::ClockTick => {
                self.datetime = Local::now();
                Task::none()
            }
        }
    }
    pub fn subscription(&self) -> Subscription<MenuMessage> {
        debug!("timestamp event emitting");
        every(Duration::from_millis(60000)).map(|_| MenuMessage::ClockTick)
    }
}
