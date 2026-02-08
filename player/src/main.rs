use cpal::{Device, traits::HostTrait};

use crate::application::Application;
use iced::{Color, Element, Theme};

mod application;
mod controls;
mod library;
mod now_playing;

fn setup_audio_output() -> Option<Device> {
    let host = cpal::default_host();
    host.default_output_device()
}

fn main() -> iced::Result {
    // First things first the player will need to setup it's audio interfaces and get ready for playback
    let device = setup_audio_output().expect("unable to setup audio device");

    // Grab the top songs from the existing catalog.

    // Wait for user input in the UI to select a song and start playback.

    /*
        let source = std::fs::File::open("path").expect("failed to open media");
        let stream = MediaSourceStream::new(Box::new(source), Default::default());

        let mut hint = Hint::new();

        let options_metadata: MetadataOptions = Default::default();
        let options_format: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, stream, &options_format, &options_metadata)
            .expect("unsupported format");

        // Next we will call DBUS to get information about what songs are on our device, the default display will
        // need be alphabetical.
        //let music = FlacReader::try_new(source, options)
    */

    iced::application(Application::new, Application::update, Application::view)
        .theme(theme)
        .decorations(false)
        .antialiasing(true)
        .run()
}

fn theme(_: &Application) -> Theme {
    iced::Theme::custom(
        String::from("Custom"),
        iced::theme::Palette {
            background: Color::WHITE,
            primary: Color::WHITE,
            text: Color::BLACK,
            success: Color::WHITE,
            warning: Color::TRANSPARENT,
            danger: Color::TRANSPARENT,
        },
    )
}
