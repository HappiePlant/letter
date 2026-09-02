use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub urls: AuthorUrls,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorUrls {
    pub website: Option<String>,
}
