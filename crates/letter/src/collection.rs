use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Collection metadata
pub struct Collection {
    pub name: String,
    pub types: Vec<CollectionType>,
    pub urls: CollectionUrls,
}

#[derive(Serialize, Deserialize)]
pub enum CollectionType {
    /// Collection contains fonts specifically made for it
    Foundry,
    /// Collection curates miscellaneous fonts
    Library,
    /// Generic collection type, i.e. a list of fonts used in a specific project
    List,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionUrls {
    /// Official website of collection
    pub website: Option<String>,
    /// Source code repository of collection
    pub source: Option<String>,
}
