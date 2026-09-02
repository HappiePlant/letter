use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
/// Font author metadata
pub struct Author {
    pub name: String,
    pub urls: AuthorUrls,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthorUrls {
    /// Personal website of the author
    pub website: Option<String>,
}
