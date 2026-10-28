use std::collections::HashMap;

use letter_core::font::{Font, FontStyle};

use crate::{
    args::{CliFontFormat, FontArgs, InstallFontArgs, SearchArgs, UpdateFontArgs},
    config::read_config,
    config_file::Config,
    network::{download_font_file, RepoClient},
    os,
    store::{self, StoredFont},
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

    async fn install_font(
        font: &Font,
        repo_id: &str,
        format: Option<CliFontFormat>,
        version: Option<String>,
        variable: bool,
        r#static: bool,
    ) {
        let version = match version {
            Some(number) => font.versions.iter().find(|v| v.number == number),
            None => font.versions.first(),
        }
        .expect("Font has no versions");

        if variable && r#static {
            panic!("Can't install both variable and static variants");
        }

        let mut files = version.files.clone();
        match format {
            Some(format) => {
                files.retain(|f| f.format == format.to_font_format());
            }
            None => {}
        }
        if variable {
            files.retain(|f| f.style == FontStyle::Variable);
        }
        if r#static {
            files.retain(|f| f.style == FontStyle::Static);
        }

        let version_file = files
            .first()
            .expect("No matching files found for the specified options");

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
        let store_path =
            store::store_font(version, &font.id, repo_id, &version_file, file.unwrap()).unwrap();
        os::install_font(&font.id, repo_id, store_path);
    }

    pub(crate) async fn install_font_cmd(&self, args: InstallFontArgs) {
        let mut valid_options = Vec::new();
        if args.repo.is_some() {
            let repo_id = args.repo.clone().unwrap();
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
                Self::install_font(
                    font,
                    repo_id,
                    args.format,
                    args.version,
                    args.variable,
                    args.r#static,
                )
                .await;
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

    pub(crate) async fn search_fonts_cmd(&self, args: SearchArgs) {
        let mut fonts: Vec<(String, Vec<Font>)> = Vec::new();
        if args.repo.is_some() {
            let repo_id = args.repo.clone().unwrap();
            let client = self.clients.get(&repo_id).expect("Repo not found");
            match client.search_fonts(args.query.as_deref()).await {
                Ok(f) => {
                    fonts.push((repo_id, f));
                }
                Err(_) => {}
            }
        } else {
            for (repo_id, client) in self.clients.iter() {
                match client.search_fonts(args.query.as_deref()).await {
                    Ok(f) => {
                        fonts.push((repo_id.clone(), f));
                    }
                    Err(_) => {}
                }
            }
        }
        for (repo_id, fonts) in fonts {
            println!("Repo: {}", repo_id);
            for font in fonts {
                println!("\tFont: {}, ID: {}", font.name, font.id);
            }
        }
    }

    async fn update_font(&self, font: &StoredFont) {
        let client = self.clients.get(&font.repo_id).expect("Repo not found");
        match client.get_font(&font.id).await {
            Ok(remote_font) => {
                let latest_version = remote_font.versions.first().expect("Font has no versions");
                if latest_version.number != font.version {
                    println!(
                        "Updating font: {} from version: {} to version: {}",
                        font.id, font.version, latest_version.number
                    );
                    store::remove_font(&font.id, &font.repo_id).unwrap();
                    os::remove_font(&font.id, &&font.repo_id);
                    // TODO: Make font update respect previous selections
                    Self::install_font(&remote_font, &font.repo_id, None, None, false, false).await;
                } else {
                    println!("Font: {} is already up to date", font.id);
                }
            }
            Err(_) => {
                println!("Failed to fetch font info for: {}", font.id);
            }
        }
    }

    pub(crate) async fn update_fonts_cmd(&self, args: UpdateFontArgs) {
        let matched_fonts: Vec<StoredFont> = match (args.id.clone(), args.repo.clone()) {
            (None, None) => store::list_fonts(),
            (None, Some(repo_id)) => store::find_fonts_in_repo(None, &repo_id).unwrap(),
            (Some(font_id), None) => {
                vec![self.find_font(&font_id).unwrap()]
            }
            (Some(font_id), Some(repo_id)) => {
                store::find_fonts_in_repo(Some(&font_id), &repo_id).unwrap()
            }
        };
        for font in matched_fonts {
            self.update_font(&font).await;
        }
    }
}
