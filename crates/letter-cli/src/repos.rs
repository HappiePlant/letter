use std::collections::HashMap;

use letter_core::font::Font;

use crate::{
    config::read_config,
    config_file::Config,
    font::{FontArgs, StoredFont},
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
        if args.repo.is_some() {
            let repo_id = args.repo.unwrap();
            let client = self.clients.get(&repo_id).expect("Repo not found");
            match client.get_font(args.id.as_str()).await {
                Ok(font) => {
                    valid_options.push((repo_id, font));
                }
                Err(_) => {}
            }
        } else {
            for (repo_id, client) in self.clients.iter() {
                match client.get_font(args.id.as_str()).await {
                    Ok(font) => {
                        valid_options.push((repo_id.clone(), font));
                    }
                    Err(_) => {}
                }
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
        let stored_fonts = store::list_fonts();
        let installed_fonts = os::list_fonts();
        for font in stored_fonts {
            let is_enabled = installed_fonts
                .iter()
                .any(|f| f.id == font.id && f.repo_id == font.repo_id);
            println!(
                "Font: {}, Repo: {}, Version: {}, Enabled: {}",
                font.id, font.repo_id, font.version, is_enabled
            );
        }
    }

    fn find_font(&self, font_id: &str) -> Result<StoredFont, ()> {
        let fonts = store::find_font(font_id).unwrap();
        match fonts.len() {
            0 => {
                panic!("Font not found in any repo");
            }
            1 => Ok(fonts.first().unwrap().clone()),
            _ => {
                panic!("Multiple fonts found, please specify a repo");
            }
        }
    }

    pub(crate) async fn remove_font_cmd(&self, args: FontArgs) {
        let repo_id = args
            .repo
            .unwrap_or_else(|| self.find_font(&args.id).unwrap().repo_id);
        store::remove_font(&args.id, &repo_id).unwrap();
        os::remove_font(&args.id, &repo_id);
    }

    pub(crate) async fn disable_font_cmd(&self, args: FontArgs) {
        let repo_id = args
            .repo
            .unwrap_or_else(|| self.find_font(&args.id).unwrap().repo_id);
        os::remove_font(&args.id, &repo_id);
    }

    pub(crate) async fn enable_font_cmd(&self, args: FontArgs) {
        let repo_id = args
            .repo
            .unwrap_or_else(|| self.find_font(&args.id).unwrap().repo_id);
        let store_path = store::get_store_path(&repo_id, &args.id);
        os::install_font(&args.id, &repo_id, store_path);
    }
}
