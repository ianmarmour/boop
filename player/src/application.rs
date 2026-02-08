use iced::{
    Color, Element, Padding, Task, Theme, border, widget::container, widget::container::Style,
};

use crate::library::{Library, LibraryMessage};

#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    LibraryMessage(LibraryMessage),
}

#[derive(Debug, Default, Clone)]
pub struct Application {
    pub library: Library,
}

impl Application {
    pub fn new() -> Application {
        Application {
            library: Library::default(),
        }
    }

    pub fn view(&self) -> Element<ApplicationMessage> {
        container(self.library.view().map(ApplicationMessage::LibraryMessage))
            .padding(Padding {
                top: 0.0,
                bottom: 0.0,
                left: 1.0,
                right: 1.0,
            })
            .center_x(480)
            .height(640)
            .width(480)
            .style(borderd_container)
            .into()
    }

    pub fn update(&mut self, message: ApplicationMessage) -> Task<ApplicationMessage> {
        match message {
            ApplicationMessage::LibraryMessage(message) => self
                .library
                .update(message)
                .map(ApplicationMessage::LibraryMessage),
        }
    }
}

pub fn borderd_container(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.base.color.into()),
        text_color: Some(palette.background.weak.text),
        border: border::rounded(0.0)
            .width(1.0) // Missing width makes border invisible
            .color(Color::BLACK), // Ensure color is applied last to be sure
        ..Style::default()
    }
}
