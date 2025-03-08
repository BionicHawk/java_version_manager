use serde::{self, Deserialize};


#[derive(Deserialize)]
pub struct Tag {
    pub name: String,
    pub node_id: String,
    pub zipball_url: String,
    pub tarball_url: String,
    pub commit: Commit,
}

#[derive(Deserialize)]
pub struct Commit {
    pub sha: String,
    pub url: String,
}

pub struct CommonFilters {
    pub page: u32,
    pub per_page: u8,
}