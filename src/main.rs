use std::sync::Arc;

use cpal::{Device, traits::HostTrait};
use repository::{artist::ArtistRepository, release::ReleaseRepository};
use serde::Serialize;
use sqlx::pool::PoolOptions;
use tokio::sync::Mutex;

use iced::{Color, Element, Theme};

use crate::{
    repository::RepositoryContext,
    service::{
        ServiceContext, artist::ArtistService, release::ReleaseService, track::TrackService,
    },
};

pub mod frontend;
pub mod model;
pub mod repository;
pub mod service;

fn setup_audio_output() -> Option<Device> {
    let host = cpal::default_host();
    host.default_output_device()
}

#[tokio::main]
async fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    // First things first the player will need to setup it's audio interfaces and get ready for playback
    let device = setup_audio_output().expect("unable to setup audio device");

    // Setup default database driver for install.
    sqlx::any::install_default_drivers();

    // Setup our database connection pool to sqlite.
    let database_pool = PoolOptions::new()
        .max_connections(5)
        .connect("sqlite:database.db?mode=rwc")
        .await
        .expect("error initializing database");

    // Setup our services context
    let repository_context = RepositoryContext::new(database_pool)
        .await
        .expect("error initializing repositories");

    let service_context = ServiceContext::new(repository_context)
        .await
        .expect("error initializing services");

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

    iced::application(
        move || frontend::application::Application::new(service_context.clone()),
        frontend::application::Application::update,
        frontend::application::Application::view,
    )
    .theme(theme)
    .decorations(false)
    .antialiasing(true)
    .subscription(frontend::application::Application::subscription)
    .run()
}

fn theme(_: &frontend::application::Application) -> Theme {
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
