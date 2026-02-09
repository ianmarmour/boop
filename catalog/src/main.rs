use std::sync::Arc;

use sqlx::pool::PoolOptions;
use tokio::sync::Mutex;
use zbus::connection;

use crate::{repository::artist::ArtistRepository, service::artist::ArtistService};

mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() {
    // Setup default database driver for install.
    sqlx::any::install_default_drivers();

    let database_pool = PoolOptions::new()
        .max_connections(5)
        .connect("sqlite:database.db?mode=rwc")
        .await
        .expect("error initializing database");

    let artist_repository = ArtistRepository::new(database_pool);
    let artist_service = ArtistService::new(Arc::new(Mutex::new(artist_repository)));
    let artist_service_connection = connection::Builder::session()
        .expect("could not build session")
        .name("org.boop.artist")
        .expect("could not build name")
        .serve_at("/org/boop/artist", artist_service)
        .expect("could not serve at desired location")
        .build()
        .await;

    tokio::signal::ctrl_c().await.unwrap();

    // On every startup of the application we should do a full filesystem traversal to detect any changes or updates to the media
    // files that are on the device.

    // We should then update the database with relevant changes to files.

    // We then need to startup our dbus serverice that will expose a handful of methods for listing songs, updating a song, getting a song, etc...
}
