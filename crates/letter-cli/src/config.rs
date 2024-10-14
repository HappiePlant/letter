use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use directories::ProjectDirs;
use crate::config_file::Config;

fn get_config_path() -> PathBuf {
    ProjectDirs::from("", "", "letter")
        .unwrap()
        .config_dir()
        .join("config.toml")
}

pub(crate) fn read_config() -> Config {
    let config_path = get_config_path();

    match fs::read_to_string(&config_path) {
        Ok(contents) => toml::from_str(&contents)
            .unwrap_or_else(|_| Config::new()),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => Config::new(),
                _ => panic!()
            }
        }
    }
}

pub(crate) fn write_config(config: &Config) {
    let config_path = get_config_path();

    let config = toml::to_string_pretty(config).expect("Failed to serialize config");

    fs::create_dir_all(config_path.parent().unwrap()).expect("Failed to create directory");
    fs::write(config_path, config).expect("Failed to write config");
}
