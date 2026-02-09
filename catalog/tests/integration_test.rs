use catalog::{
    model::{CatalogItem, artist::Artist},
    repository::artist::ArtistFilter,
};
use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;
use zbus::Connection;

#[tokio::test]
async fn test_get_artist() {
    let _server = Command::new("cargo")
        .args(["run", "--bin", "catalog"])
        .spawn()
        .expect("failed to start server");

    let connection = Connection::session()
        .await
        .expect("failed to connect to dbus");

    // Wait for service to be ready
    for _ in 0..50 {
        let proxy = zbus::Proxy::new(
            &connection,
            "org.boop.artist",
            "/org/boop/artist",
            "org.boop.artist",
        )
        .await
        .expect("failed to create proxy");

        let filter = ArtistFilter::default();

        // Debug: print what signature we're sending
        println!(
            "Filter signature: {:?}",
            <ArtistFilter as zbus::zvariant::Type>::SIGNATURE
        );

        let result: Result<Vec<CatalogItem<Artist>>, _> =
            proxy.call("ListArtists", &(filter,)).await;

        println!("Result: {:?}", result);

        sleep(Duration::from_millis(100)).await;
    }

    panic!("service did not start in time");
}
