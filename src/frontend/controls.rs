use iced::{
    Element,
    widget::{button, row},
};

#[derive(Default, Debug, Clone, Copy)]
pub enum PlayButtonState {
    #[default]
    Playing,
    Paused,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayButtonMessage {
    Pressed,
}

#[derive(Default)]
struct PlayButton {
    state: PlayButtonState,
}

impl PlayButton {
    pub fn update(&mut self, message: PlayButtonMessage) {
        match message {
            PlayButtonMessage::Pressed => {
                self.state = match self.state {
                    PlayButtonState::Paused => PlayButtonState::Playing,
                    PlayButtonState::Playing => PlayButtonState::Paused,
                }
            }
        }
    }

    pub fn view(&self) -> Element<PlayButtonMessage> {
        match self.state {
            PlayButtonState::Paused => button("⏸")
                .width(250)
                .on_press(PlayButtonMessage::Pressed)
                .into(),
            PlayButtonState::Playing => button("▶")
                .width(250)
                .on_press(PlayButtonMessage::Pressed)
                .into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NextButtonMessage {
    Pressed,
}

#[derive(Default)]
struct NextButton {
    pressed: bool,
}

impl NextButton {
    pub fn update(&mut self, message: NextButtonMessage) {
        match message {
            NextButtonMessage::Pressed => self.pressed = true,
        }
    }

    pub fn view(&self) -> Element<NextButtonMessage> {
        button(">").on_press(NextButtonMessage::Pressed).into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PrevButtonMessage {
    Pressed,
}

#[derive(Default)]
struct PrevButton {
    pressed: bool,
}

impl PrevButton {
    pub fn update(&mut self, message: PrevButtonMessage) {
        match message {
            PrevButtonMessage::Pressed => self.pressed = true,
        }
    }

    pub fn view(&self) -> Element<PrevButtonMessage> {
        button("<").on_press(PrevButtonMessage::Pressed).into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ControlsMessage {
    PrevButtonMessage(PrevButtonMessage),
    PlayButtonMessage(PlayButtonMessage),
    NextButtonMessage(NextButtonMessage),
}

#[derive(Default)]
pub struct Controls {
    prev_button: PrevButton,
    play_button: PlayButton,
    next_button: NextButton,
}

impl Controls {
    pub fn update(&mut self, message: ControlsMessage) {
        match message {
            ControlsMessage::NextButtonMessage(msg) => self.next_button.update(msg),
            ControlsMessage::PlayButtonMessage(msg) => self.play_button.update(msg),
            ControlsMessage::PrevButtonMessage(msg) => self.prev_button.update(msg),
        }
    }

    pub fn view(&self) -> Element<ControlsMessage> {
        row![
            self.prev_button
                .view()
                .map(ControlsMessage::PrevButtonMessage),
            self.play_button
                .view()
                .map(ControlsMessage::PlayButtonMessage),
            self.next_button
                .view()
                .map(ControlsMessage::NextButtonMessage)
        ]
        .spacing(10)
        .into()
    }
}
