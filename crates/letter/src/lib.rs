use serde::{Deserialize, Serialize};

pub mod author;
pub mod collection;
pub mod endpoints;
pub mod font;

#[derive(Serialize, Deserialize)]
pub enum UriType {
    External,
    Letter,
}
