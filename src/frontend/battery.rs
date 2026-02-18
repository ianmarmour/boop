use battery::units::ratio::percent;
use iced::{
    Background, Border, Color, Element, Task, widget::progress_bar, widget::progress_bar::Style,
};

use crate::battery::BatteryHandle;

#[derive(Debug, Clone)]
pub enum BatteryMessage {
    Tick,
}

#[derive(Debug, Clone)]
pub struct Battery {
    handle: BatteryHandle,
}

impl Battery {
    pub fn new() -> Self {
        Self {
            handle: BatteryHandle::new().expect("something went horribly wrong"),
        }
    }

    pub fn view(&self) -> Element<'_, BatteryMessage> {
        progress_bar(0.0..=100.0, self.handle.charge().get::<percent>())
            .length(50)
            .girth(30)
            .style(|_| Style {
                background: Background::Color(Color::BLACK),
                bar: Background::Color(Color::WHITE),
                border: Border::default().width(5).color(Color::WHITE),
            })
            .into()
    }

    pub fn update(&mut self, message: BatteryMessage) -> Task<BatteryMessage> {
        match message {
            BatteryMessage::Tick => {
                let _ = self.handle.refresh();
                Task::none()
            }
        }
    }
}
