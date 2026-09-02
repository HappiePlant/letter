use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod author;
pub mod collection;
pub mod font;

#[cfg(feature = "generate-openapi")]
#[allow(dead_code)]
pub mod openapi;

#[derive(Serialize, Deserialize, ToSchema)]
/// Repo metadata
pub struct RepoInfo {
    /// Name of the repo
    pub name: String,
    /// Short description of the repos content
    pub description: String,
    /// Recommended ID for the repo, can be customized by users
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UriType {
    /// External link
    External,
    /// Link to a letter resource
    Letter,
}
