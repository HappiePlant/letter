use serde::{Deserialize, Serialize};

use crate::{UriType, author::Author, collection::Collection};

#[derive(Serialize, Deserialize)]
pub struct Font {
    id: String,
    name: String,
    authors: Vec<Author>,
    foundry: Option<CollectionRelation>,
    licenses: Vec<String>,
    fork_of: Option<ForkOf>,
    urls: FontUrls,
    collections: Vec<CollectionRelation>,
    versions: Vec<Version>,
}

#[derive(Serialize, Deserialize)]
pub struct ForkOf {
    uri: String,
    r#type: UriType,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FontUrls {
    website: Option<String>,
    source: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    number: String,
    urls: Vec<String>,
    hash: String,
    r#type: FileType,
    files: Vec<VersionFile>,
}

#[derive(Serialize, Deserialize)]
pub enum FileType {
    Zip,
    SingleFile,
}

#[derive(Serialize, Deserialize)]
pub struct VersionFile {
    pub glob: String,
    pub style: FontStyle,
    pub format: FontFormat,
}

#[derive(Serialize, Deserialize)]
pub enum FontFormat {
    OTF,
    TTF,
    WOFF,
    WOFF2,
    TTC,
}

#[derive(Serialize, Deserialize)]
pub enum FontStyle {
    Variable,
    Static,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionRelation {
    url: Option<String>,
    collection: Collection,
}
