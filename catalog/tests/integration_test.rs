use catalog::{
    client::artist::ArtistServiceProxy,
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

    let client = ArtistServiceProxy::new(&connection)
        .await
        .expect("unable to create client");

    // Wait for service to be ready
    for _ in 0..50 {
        let filter = ArtistFilter::default();

        // Debug: print what signature we're sending
        println!(
            "Filter signature: {:?}",
            <ArtistFilter as zbus::zvariant::Type>::SIGNATURE
        );

        let result = client.list_artists(filter).await;

        match result {
            Ok(_) => return,
            Err(_) => sleep(Duration::from_millis(100)).await,
        }
    }

    panic!("service did not start in time");
}
