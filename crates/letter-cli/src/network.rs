use std::sync::LazyLock;

use letter_core::{font::Font, RepoInfo};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};

use crate::error::Error;

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub(crate) struct RepoClient {
    pub(crate) base_url: String,
}

impl RepoClient {
    async fn handle_response<T: DeserializeOwned>(res: Response) -> Result<T, Error> {
        match res.status() {
            StatusCode::OK => Ok(res.json().await?),
            StatusCode::NOT_FOUND => Err(Error::RepoFetchNotFound),
            _ => Err(Error::RepoFetchError),
        }
    }

    pub(crate) async fn get_repo_info(&self) -> Result<RepoInfo, Error> {
        let response = CLIENT.get(&self.base_url).send().await?;
        Self::handle_response(response).await
    }

    pub(crate) async fn get_font(&self, font_id: &str) -> Result<Font, Error> {
        let url = format!("{}fonts/{}", self.base_url, font_id);
        let response = CLIENT.get(&url).send().await?;
        Self::handle_response(response).await
    }

    pub(crate) async fn search_fonts(&self, query: Option<&str>) -> Result<Vec<Font>, Error> {
        let url = match query {
            Some(q) => format!("{}fonts?query={}", self.base_url, q),
            None => format!("{}fonts", self.base_url),
        };
        let response = CLIENT.get(&url).send().await?;
        Self::handle_response(response).await
    }
}

pub(crate) async fn download_font_file(url: &str, expected_hash: &str) -> Result<Vec<u8>, Error> {
    let response = CLIENT.get(url).send().await?;
    let bytes = response.bytes().await?;
    let actual_hash = Sha256::digest(bytes.clone());
    let decoded_expected_hash = hex::decode(expected_hash)?;
    if actual_hash.as_slice() != decoded_expected_hash.as_slice() {
        Err(Error::HashMismatch)
    } else {
        Ok(bytes.to_vec())
    }
}
