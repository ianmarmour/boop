use std::sync::Arc;

use sqlx::pool::PoolOptions;
use tokio::sync::Mutex;
use zbus::connection;

use catalog::{repository::artist::ArtistRepository, service::artist::ArtistService};

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

    // Setup our various DBUS services and repositories.
    let artist_repository = ArtistRepository::new(database_pool);
    let artist_service = ArtistService::new(Arc::new(Mutex::new(artist_repository)));
    let _ = connection::Builder::session()
        .expect("could not build session")
        .name("org.boop.artist")
        .expect("could not build name")
        .serve_at("/org/boop/artist", artist_service)
        .expect("could not serve at desired location")
        .build()
        .await;

    tokio::signal::ctrl_c().await.unwrap();
}
