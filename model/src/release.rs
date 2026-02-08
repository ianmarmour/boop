#[derive(Debug, Clone)]
pub struct Release {
    pub id: u64,
    pub name: String,
    pub tracks: Vec<u64>,
    pub artist: u64,
}
