#[derive(Debug, Clone)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub releases: Vec<u64>,
}
