use std::path::PathBuf;

use cpal::{Device, traits::HostTrait};
use sqlx::{pool::PoolOptions, sqlite::SqlitePoolOptions};

use iced::{Color, Font, Pixels, Settings, Size, Theme};

use crate::{repository::RepositoryContext, service::CatalogService};

pub mod audio;
pub mod battery;
pub mod frontend;
pub mod model;
pub mod repository;
pub mod service;

const APPLICATION_FONT: &[u8] = include_bytes!("resources/jersey_regular.ttf");

fn setup_audio_output() -> Option<Device> {
    let host = cpal::default_host();
    host.default_output_device()
}

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    let _ = setup_audio_output().expect("unable to setup audio device");

    // Create a runtime just for setup
    let rt = tokio::runtime::Runtime::new().unwrap();

    let catalog_service = rt.block_on(async {
        sqlx::any::install_default_drivers();

        let database_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:database.db?mode=rwc")
            .await
            .expect("error initializing database");

        let repository_context = RepositoryContext::new(database_pool)
            .await
            .expect("error initializing repositories");

        let service = CatalogService::new(repository_context)
            .await
            .expect("error initializing services");

        let _ = service
            .sync(PathBuf::from("/Users/ian/Desktop/music/"))
            .await;

        return service;
    });

    // Drop the runtime before Iced creates its own
    drop(rt);

    iced::application(
        move || frontend::application::Application::new(catalog_service.clone()),
        frontend::application::Application::update,
        frontend::application::Application::view,
    )
    .font(APPLICATION_FONT)
    .theme(theme)
    .decorations(true)
    .antialiasing(true)
    .window_size(Size::new(720.0, 720.0))
    .subscription(frontend::application::Application::subscription)
    .settings(Settings {
        default_text_size: Pixels(48.0),
        default_font: Font::with_name("Jersey 10"),
        fonts: vec![APPLICATION_FONT.into()],
        ..Default::default()
    })
    .resizable(false)
    .run()
}

fn theme(_: &frontend::application::Application) -> Theme {
    iced::Theme::custom(
        String::from("Custom"),
        iced::theme::Palette {
            background: Color::BLACK,
            primary: Color::BLACK,
            text: Color::WHITE,
            success: Color::WHITE,
            warning: Color::TRANSPARENT,
            danger: Color::TRANSPARENT,
        },
    )
}
