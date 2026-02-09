use catalog::{
    model::{CatalogItem, artist::Artist},
    repository::artist::ArtistFilter,
};
use tokio::process::Command;
use zbus::Connection;

#[tokio::test]
async fn test_get_artist() {
    let server = Command::new("cargo")
        .args(["run", "--bin", "catalog"])
        .spawn()
        .expect("failed to start server");

    let connection = Connection::session()
        .await
        .expect("failed to connect to dbus");

    let proxy = zbus::Proxy::new(
        &connection,
        "org.boop.artist",
        "/org/boop/artist",
        "org.boop.artist",
    )
    .await
    .expect("failed to create dbus proxy");

    // List all artists
    println!("Listing all artists:");
    let result: Result<Vec<CatalogItem<Artist>>, _> =
        proxy.call("ListArtists", &(ArtistFilter::default(),)).await;

    match result {
        Ok(artists) => {
            if artists.is_empty() {
                println!("  No artists found");
            }
            for artist in &artists {
                println!("  id: {}, metadata: {:?}", artist.id, artist.metadata);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Get specific artist
    println!("\nSearching for 'Beatles':");
    let result: Result<CatalogItem<Artist>, _> = proxy.call("GetArtist", &("Beatles",)).await;

    match result {
        Ok(artist) => println!("  Found: id={}, metadata={:?}", artist.id, artist.metadata),
        Err(e) => println!("  Not found: {}", e),
    }
}
