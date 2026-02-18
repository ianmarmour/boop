use std::time::Duration;

use battery::units::{Ratio, ratio::percent};
use chrono::{DateTime, Local, Utc};
use iced::{
    Element, Length, Padding, Subscription, Task,
    alignment::Horizontal,
    time::every,
    widget::{Column, row, text},
};
use tracing::debug;

use crate::{
    battery::BatteryHandle,
    frontend::{
        application::ApplicationView,
        battery::{Battery, BatteryMessage},
        player::PlayerState,
    },
};

#[derive(Debug, Clone)]
pub enum MenuMessage {
    ViewChange(ApplicationView),
    Battery(BatteryMessage),
    Tick,
}

#[derive(Debug, Clone)]
pub struct Menu {
    current_view: ApplicationView,
    datetime: DateTime<Local>,
    player_status: Option<PlayerState>,
    battery: Battery,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            current_view: ApplicationView::default(),
            datetime: Local::now(),
            player_status: None,
            battery: Battery::new(),
        }
    }
}

impl Menu {
    pub fn new(current_view: ApplicationView) -> Self {
        Self {
            current_view,
            datetime: Local::now(),
            player_status: None,
            battery: Battery::new(),
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
            Column::new()
                .push(self.battery.view().map(MenuMessage::Battery))
                .width(Length::FillPortion(1))
                .padding(Padding::new(0.0).top(15))
                .align_x(Horizontal::Right),
        ]
        .padding(Padding::new(0.0).horizontal(10.0))
        .height(50)
        .into()
    }
    pub fn update(&mut self, message: MenuMessage) -> Task<MenuMessage> {
        match message {
            MenuMessage::ViewChange(view) => {
                self.current_view = view;
                Task::none()
            }
            MenuMessage::Tick => {
                self.datetime = Local::now();
                self.battery
                    .update(BatteryMessage::Tick)
                    .map(MenuMessage::Battery)
            }
            MenuMessage::Battery(message) => self.battery.update(message).map(MenuMessage::Battery),
        }
    }
    pub fn subscription(&self) -> Subscription<MenuMessage> {
        debug!("timestamp event emitting");
        every(Duration::from_millis(60000)).map(|_| MenuMessage::Tick)
    }
}
