use std::sync::Arc;

use sqlx::pool::PoolOptions;
use tokio::sync::Mutex;
use zbus::connection;

use catalog::{
    ApplicationState,
    repository::{artist::ArtistRepository, release::ReleaseRepository},
    service::{artist::ArtistService, release::ReleaseService},
};

#[tokio::main]
async fn main() {
    // Setup default database driver for install.
    sqlx::any::install_default_drivers();

    // Setup our database connection pool to sqlite.
    let database_pool = PoolOptions::new()
        .max_connections(5)
        .connect("sqlite:database.db?mode=rwc")
        .await
        .expect("error initializing database");

    // Setup our shared application state.
    let state = ApplicationState {
        artists: Arc::new(Mutex::new(
            ArtistRepository::new(database_pool.clone())
                .await
                .expect("error initializing artist repository"),
        )),
        releases: Arc::new(Mutex::new(
            ReleaseRepository::new(database_pool.clone())
                .await
                .expect("error initializing artist repository"),
        )),
    };

    // Setup our services.
    let artist_service = ArtistService::new(state.clone());
    let release_service = ReleaseService::new(state.clone());

    // Setup our DBUS connection
    let _connection = connection::Builder::session()
        .expect("could not build session")
        .name("org.boop.catalog")
        .expect("could not build name")
        .serve_at("/org/boop/catalog/artist", artist_service)
        .expect("could not serve artist")
        .serve_at("/org/boop/catalog/release", release_service)
        .expect("could not serve release")
        .build()
        .await
        .expect("could not build connection");

    tokio::signal::ctrl_c().await.unwrap();
}
