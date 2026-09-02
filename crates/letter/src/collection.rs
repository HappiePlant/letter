use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub types: Vec<CollectionType>,
    pub urls: CollectionUrls,
}

#[derive(Serialize, Deserialize)]
pub enum CollectionType {
    Foundry,
    Library,
    List,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionUrls {
    pub website: Option<String>,
    pub source: Option<String>,
}
