use std::time::Duration;

use chrono::{DateTime, Utc};
use iced::{
    Element, Length, Padding, Subscription, Task,
    time::every,
    widget::{Column, Row, row, text},
};
use tracing::debug;

use crate::frontend::player::PlayerState;

#[derive(Debug, Clone)]
pub enum MenuMessage {
    PlayerStatus(PlayerState),
    ClockTick,
}

#[derive(Debug, Clone)]
pub struct Menu {
    datetime: DateTime<Utc>,
    player_status: Option<PlayerState>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            datetime: Utc::now(),
            player_status: None,
        }
    }
}

impl Menu {
    pub fn new() -> Self {
        Self {
            datetime: Utc::now(),
            player_status: None,
        }
    }
    pub fn view(&self) -> Element<'_, MenuMessage> {
        row![
            Column::new().push(text(self.datetime.format("%H:%M").to_string())),
            Column::new().width(Length::Fill),
            Column::new().push(text(
                self.player_status
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            ))
        ]
        .padding(Padding::new(0.0).horizontal(5.0))
        .into()
    }
    pub fn update(&mut self, message: MenuMessage) -> Task<MenuMessage> {
        match message {
            MenuMessage::PlayerStatus(status) => {
                self.player_status = Some(status);
                Task::none()
            }
            MenuMessage::ClockTick => {
                self.datetime = Utc::now();
                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn subscription(&self) -> Subscription<MenuMessage> {
        debug!("timestamp event emitting");
        every(Duration::from_millis(100)).map(|_| MenuMessage::ClockTick)
    }
}
