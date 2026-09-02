use crate::{author::Author, collection::Collection, font::Font};

pub struct CollectionResponse {
    pub collection: Collection,
    pub fonts: Vec<Font>,
}

pub struct AuthorResponse {
    pub author: Author,
    pub fonts: Vec<Font>,
}
