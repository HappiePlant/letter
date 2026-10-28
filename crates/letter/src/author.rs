use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
/// Font author metadata
pub struct Author {
    pub id: String,
    pub name: String,
    pub urls: AuthorUrls,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AuthorUrls {
    /// Personal website of the author
    pub website: Option<String>,
}
