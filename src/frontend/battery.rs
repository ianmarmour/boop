use battery::units::ratio::percent;
use iced::{
    Background, Border, Color, Element, Task,
    widget::{Space, progress_bar, progress_bar::Style},
};

use crate::battery::{BatteryHandle, BatteryHandleError};

#[derive(Debug, Clone)]
pub enum BatteryMessage {
    Tick,
}

#[derive(Debug, Clone)]
pub struct Battery {
    handle: Option<BatteryHandle>,
}

impl Battery {
    pub fn new() -> Self {
        let handle = match BatteryHandle::new() {
            Ok(h) => Some(h),
            Err(BatteryHandleError::BatteryNotFound) => None,
            Err(e) => {
                tracing::warn!("failed to initialise battery handle: {e}");
                None
            }
        };
        Self { handle }
    }

    /// Returns `true` when a physical battery was detected on this system.
    pub fn has_battery(&self) -> bool {
        self.handle.is_some()
    }

    pub fn view(&self) -> Element<'_, BatteryMessage> {
        match &self.handle {
            Some(handle) => progress_bar(0.0..=100.0, handle.charge().get::<percent>())
                .length(50)
                .girth(30)
                .style(|_| Style {
                    background: Background::Color(Color::BLACK),
                    bar: Background::Color(Color::WHITE),
                    border: Border::default().width(5).color(Color::WHITE),
                })
                .into(),
            None => Space::new().into(),
        }
    }

    pub fn update(&mut self, message: BatteryMessage) -> Task<BatteryMessage> {
        match message {
            BatteryMessage::Tick => {
                if let Some(handle) = &mut self.handle {
                    let _ = handle.refresh();
                }
                Task::none()
            }
        }
    }
}
