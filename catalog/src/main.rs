fn main() {
    println!("Hello, world!");

    // On every startup of the application we should do a full filesystem traversal to detect any changes or updates to the media
    // files that are on the device.

    // We should then update the database with relevant changes to files.

    // We then need to startup our dbus serverice that will expose a handful of methods for listing songs, updating a song, getting a song, etc...
}
