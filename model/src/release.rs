use crate::Id;

#[derive(Debug, Clone)]
pub struct Release {
    pub id: u64,
    pub name: String,
    pub tracks: Vec<u64>,
    pub artist: u64,
}

impl Id for Release {
    fn id(self) -> u64 {
        self.id
    }
}
