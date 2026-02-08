use crate::Id;

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub releases: Vec<u64>,
}

impl Id for Artist {
    fn id(self) -> u64 {
        self.id
    }
}
