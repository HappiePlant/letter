use std::collections::HashMap;

use letter_core::font::Font;

use crate::{
    config::read_config,
    config_file::Config,
    font::FontArgs,
    network::{download_font_file, RepoClient},
    os, store,
};

pub(crate) struct RepoList {
    clients: HashMap<String, RepoClient>,
    config: Config,
}

impl RepoList {
    pub(crate) fn new() -> Self {
        let config = read_config().unwrap_or_else(|_| Config::new());
        let clients = config
            .repo
            .iter()
            .map(|repo| {
                (
                    repo.id.clone(),
                    RepoClient {
                        base_url: repo.url.to_string(),
                    },
                )
            })
            .collect();
        Self { clients, config }
    }

    async fn install_font(font: &Font, repo_id: &str) {
        let version = font.versions.first().expect("Font has no versions");
        let mut file = None;
        for url in &version.urls {
            match download_font_file(url, &version.hash).await {
                Ok(f) => {
                    file = Some(f);
                    break;
                }
                Err(_) => {
                    continue;
                }
            }
        }
        if file.is_none() {
            panic!("Failed to download font file");
        }
        let store_path = store::store_font(version, &font.id, repo_id, file.unwrap()).unwrap();
        os::install_font(&font.id, repo_id, store_path);
    }

    pub(crate) async fn install_font_cmd(&self, args: FontArgs) {
        let mut valid_options = Vec::new();
        for (repo_id, client) in self.clients.iter() {
            match client.get_font(args.id.as_str()).await {
                Ok(font) => {
                    valid_options.push((repo_id, font));
                }
                Err(_) => {}
            }
        }
        match valid_options.len() {
            0 => println!("Font not found in any repo"),
            1 => {
                let (repo_id, font) = &valid_options[0];
                println!("Installing font: {} from repo: {}", font.name, repo_id);
                Self::install_font(font, repo_id).await;
            }
            _ => {
                todo!("Multiple fonts found, please specify a repo");
            }
        }
    }

    pub(crate) async fn list_fonts_cmd(&self) {
        let installed_fonts = store::list_fonts();
        for font in installed_fonts {
            println!(
                "Font: {}, Repo: {}, Version: {}",
                font.id, font.repo_id, font.version
            );
        }
    }
}
