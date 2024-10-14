use url::Url;
use serde::Deserialize;
use toml;
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Deserialize)]
pub(crate) struct Config {
    repo: Vec<Repo>,
}

#[derive(Deserialize)]
struct Repo {
    id: String,
    url: String,
}

impl Config {
    fn new() -> Self {
        Self {
            repo: Vec::new(),
        }
    }
}

fn read_config() {
    let config = fs::read_to_string("config.toml");
    // match config {
    //     Ok(contents) => toml::from_str(contents.as_str()),
    // }
}
