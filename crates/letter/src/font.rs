use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{UriType, author::Author, collection::Collection};

#[derive(Serialize, Deserialize, ToSchema)]
/// Font metadata
pub struct Font {
    pub id: String,
    pub name: String,
    pub authors: Vec<Author>,
    /// Main foundry of the font
    pub foundry: Option<CollectionRelation>,
    pub licenses: Vec<String>,
    /// Optional relation to a font that this font is a fork of
    pub fork_of: Option<ForkOf>,
    pub urls: FontUrls,
    /// Collections that this font is a part of
    pub collections: Vec<CollectionRelation>,
    /// List of versions of the font, sorted from newest to oldest
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ForkOf {
    pub uri: String,
    pub r#type: UriType,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FontUrls {
    /// Official website of the font
    pub website: Option<String>,
    /// Source code repository of the font
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Version {
    pub number: String,
    /// Direct download url to the file, backup urls can be provided
    pub urls: Vec<String>,
    /// Sha256 hash of the linked file
    pub hash: String,
    pub r#type: FileType,
    /// Contents of the linked file, used for installing it
    pub files: Vec<VersionFile>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    Zip,
    SingleFile,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionFile {
    /// Glob pattern to match the files in the zip, or the single file name
    pub glob: String,
    pub style: FontStyle,
    pub format: FontFormat,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum FontFormat {
    OTF,
    TTF,
    WOFF,
    WOFF2,
    TTC,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FontStyle {
    Variable,
    Static,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CollectionRelation {
    pub url: Option<String>,
    pub collection: Collection,
}
