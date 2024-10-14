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

pub(crate) fn read_config() -> Result<Config, String> {
    let config_path = get_config_path();

    match fs::read_to_string(&config_path) {
        Ok(contents) => {
            toml::from_str(&contents)
                .unwrap_or_else(|err| Err(format!("Failed to read config file: {}", err)))
        }
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => Ok(Config::new()),
                _ => Err(err.to_string()),
            }
        }
    }
}

pub(crate) fn write_config(config: &Config) -> Result<(), String> {
    let config_path = get_config_path();

    let config = toml::to_string_pretty(config)
        .map_err(|_| "Failed to serialize config")?;

    fs::create_dir_all(config_path.parent().unwrap())
        .map_err(|_| "Failed to create config directory")?;

    fs::write(config_path, config)
        .map_err(|_| "Failed to write config file")?;
    Ok(())
}
