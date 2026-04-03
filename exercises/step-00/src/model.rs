#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildProfile {
    pub name: String,
    pub owner: Option<String>,
    pub tags: Vec<String>,
    pub retries: u8,
}

impl BuildProfile {
    pub fn new(name: String, owner: Option<String>, tags: Vec<String>, retries: u8) -> Self {
        Self {
            name,
            owner,
            tags,
            retries,
        }
    }
}
