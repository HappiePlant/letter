use std::sync::LazyLock;

use letter_core::{font::Font, RepoInfo};

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
