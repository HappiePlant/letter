// use bytes::Bytes;
use std::{str::Bytes, sync::LazyLock};

use letter_core::{font::Font, RepoInfo};
use sha2::{Digest, Sha256};

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub(crate) struct RepoClient {
    pub(crate) base_url: String,
}

impl RepoClient {
    pub(crate) async fn get_repo_info(&self) -> Result<RepoInfo, reqwest::Error> {
        let response = CLIENT.get(&self.base_url).send().await?;
        let repo_info: RepoInfo = response.json().await?;
        Ok(repo_info)
    }

    pub(crate) async fn get_font(&self, font_id: &str) -> Result<Font, reqwest::Error> {
        let url = format!("{}fonts/{}", self.base_url, font_id);
        let response = CLIENT.get(&url).send().await?;
        let font: letter_core::font::Font = response.json().await?;
        Ok(font)
    }
}

pub(crate) async fn download_font_file(
    url: &str,
    expected_hash: &str,
) -> Result<Vec<u8>, reqwest::Error> {
    let response = CLIENT.get(url).send().await?;
    let bytes = response.bytes().await?;
    let actual_hash = Sha256::digest(bytes.clone());
    let decoded_expected_hash = hex::decode(expected_hash).expect("Invalid expected hash");
    if actual_hash.as_slice() != decoded_expected_hash.as_slice() {
        panic!("Hash mismatch");
    } else {
        Ok(bytes.to_vec())
    }
}
