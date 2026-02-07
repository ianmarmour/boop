# boop

## Design

## System

The operating system will be minimal linux distro stripped of all functionality that's not directly needed for render to the screen, reading or writing to the filesystem and playing audio. 

### Software

#### Services

##### Catalog

This will be a central metadata store that's responsible for containing a list of all songs and their associated metadata. This service will be started as a system-d unit and be running the background at all times. The service can and will be called by the player to display metadata and information about what songs are avaliable on the device. This service can be called explicitly by the player to refresh the files metadata on disk but also will attempt to refresh metadata after file changes have occured. The service will expose a variety of API's include but not limited to `UpdateTrackMetadata`, `GetTrackMetadata` and `ListTrackMetadata`. When we refer to track metadata we also mean metadata related to playback counts, hearts, etc not simply the metadata of the audio file though that is also included.

##### Player

The player itself will be a frontend built in iced that connects to the library over a form of RPC

The player itself should minimally include the following functionality,

* playlists for tracks
* visualization of album art
* a control system for playing, pausing, favoriting, skipping and going back to a previous track.
* visualization of playback status for a track.

by default when the device is turned off the player should store it's existing state on disk and resume in the
exact same state that it left off it's playback previously.

#### Dependencies

* [Sympohnia](https://github.com/pdeljanov/Symphonia/tree/master) - Used for playback of various file times and metadata parsing.
* [CPAL](https://github.com/RustAudio/cpal) - Used for audio IO.
* [ZBUS](https://docs.rs/zbus/latest/zbus/) - Used for library to player IPC.
* [ICED](https://github.com/iced-rs/iced) - Used for rendering/GUI.
* [RUSTQLITE](https://crates.io/crates/libsqlite3-sys) - Used for storing metadata and playback information.

### Hardware

The initial hardware design will leverage a rapsberry pi zero w 2.

#### CPU

Quad-core 64-bit ARM Cortex-A53

#### IO 

##### Audio

The device will have a single 3.5 mm audio jack and will not support any sort of USB-C audio at release.

##### Displays

The device will have a single display that is OLED and has a square aspect ratio.

##### Buttons

The device will have multiple physical hardware buttons include a play, pause, next, back and volume slider.

#### Battery

The device will have a high capacity 3000mah lithium ion battery.
