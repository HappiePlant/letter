use crate::config_file::Config;
use crate::error::Error;
use directories::ProjectDirs;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    ProjectDirs::from("", "", "letter")
        .unwrap()
        .config_dir()
        .join("config.toml")
}

pub(crate) fn read_config() -> Result<Config, Error> {
    let config_path = get_config_path();

    let contents = match fs::read_to_string(&config_path) {
        Ok(contents) => contents,
        Err(err) => {
            return match err.kind() {
                ErrorKind::NotFound => Ok(Config::new()),
                _ => Err(Error::ConfigError(err.to_string())),
            }
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            return Err(Error::ConfigError(format!(
                "Failed to parse config file: {}",
                err
            )))
        }
    };
    Ok(config)
}

pub(crate) fn write_config(config: &Config) -> Result<(), Error> {
    let config_path = get_config_path();

    let config = toml::to_string_pretty(config)?;

    fs::create_dir_all(config_path.parent().unwrap())
        .map_err(|_| Error::ConfigError("Failed to create config directory".to_string()))?;

    fs::write(config_path, config)
        .map_err(|_| Error::ConfigError("Failed to write config file".to_string()))?;
    Ok(())
}
