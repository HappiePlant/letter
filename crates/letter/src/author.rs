use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Font author metadata
pub struct Author {
    pub name: String,
    pub urls: AuthorUrls,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorUrls {
    /// Personal website of the author
    pub website: Option<String>,
}
