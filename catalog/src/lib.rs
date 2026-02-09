use std::sync::Arc;

use tokio::sync::Mutex;

use crate::repository::{artist::ArtistRepository, release::ReleaseRepository};

pub mod client;
pub mod model;
pub mod repository;
pub mod service;

#[derive(Clone)]
pub struct ApplicationState {
    pub artists: Arc<Mutex<ArtistRepository>>,
    pub releases: Arc<Mutex<ReleaseRepository>>,
}
