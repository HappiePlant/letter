use std::collections::HashMap;

use crate::{config::read_config, config_file::Config, font::FontArgs, network::RepoClient};

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

    pub(crate) async fn install_font(&self, args: FontArgs) {
        let mut valid_options = Vec::new();
        for client in self.clients.values() {
            match client.get_font(args.id.as_str()).await {
                Ok(font) => {
                    valid_options.push(font);
                }
                Err(_) => {}
            }
        }
        match valid_options.len() {
            0 => println!("Font not found in any repo"),
            1 => {
                let font = &valid_options[0];
                println!("Installing font: {}", font.name);
            }
            _ => {
                todo!("Multiple fonts found, please specify a repo");
            }
        }
    }
}
