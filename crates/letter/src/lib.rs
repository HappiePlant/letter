use serde::{Deserialize, Serialize};

pub mod author;
pub mod collection;
pub mod endpoints;
pub mod font;

/// Repo metadata
pub struct RepoInfo {
    /// Name of the repo
    pub name: String,
    /// Short description of the repos content
    pub description: String,
    /// Recommended ID for the repo, can be customized by users
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub enum UriType {
    /// External link
    External,
    /// Link to a letter resource
    Letter,
}
