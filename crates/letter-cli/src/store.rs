use std::{
    fs,
    io::{self, Cursor},
    path::PathBuf,
    sync::LazyLock,
};

use directories::ProjectDirs;
use globset::{Glob, GlobSetBuilder};
use letter_core::font::{FileType, FontStyle, Version, VersionFile};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::{args::CliFontFormat, error::Error};

#[derive(Clone, Debug)]
pub(crate) struct StoredFont {
    pub id: String,
    pub repo_id: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct FontConfig {
    pub style_preference: Option<FontStyle>,
    pub format_preference: Option<CliFontFormat>,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            format_preference: None,
            style_preference: None,
        }
    }
}

pub(crate) static STORE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    ProjectDirs::from("", "", "letter")
        .unwrap()
        .data_dir()
        .join("store")
});

pub(crate) fn get_store_path(repo_id: &str, font_id: &str) -> PathBuf {
    let dir = STORE_DIR.join(repo_id).join(font_id);
    let mut versions = fs::read_dir(&dir).unwrap();
    dir.join(versions.nth(0).unwrap().unwrap().file_name())
}

pub(crate) fn store_font(
    version: &Version,
    font_id: &str,
    repo_id: &str,
    version_file: &VersionFile,
    file: Vec<u8>,
    format_preference: Option<CliFontFormat>,
    style_preference: Option<FontStyle>,
) -> Result<PathBuf, Error> {
    let font_dir = STORE_DIR.join(repo_id).join(font_id);
    let version_dir = font_dir.join(&version.number);
    fs::create_dir_all(&version_dir)
        .map_err(|_| Error::StoreIoError("Failed to create font directory".to_string()))?;

    match version.r#type {
        FileType::Zip => {
            let cursor = Cursor::new(file);
            let mut zip = ZipArchive::new(cursor)
                .map_err(|_| Error::ZipError("Can't open zip".to_string()))?;

            let mut glob_set_builder = GlobSetBuilder::new();
            glob_set_builder.add(Glob::new(&version_file.glob)?);
            let glob_set = glob_set_builder.build()?;

            for i in 0..zip.len() {
                let mut item = zip.by_index(i).unwrap();
                let path = item
                    .enclosed_name()
                    .ok_or(Error::ZipError("Failed to read path".to_string()))?;
                if glob_set.is_match(&path) {
                    let mut out = fs::File::create(
                        version_dir.join(
                            path.file_name()
                                .ok_or(Error::ZipError("Failed to get filename".to_string()))?,
                        ),
                    )
                    .map_err(|_| Error::StoreIoError("Failed to create file".to_string()))?;
                    io::copy(&mut item, &mut out)
                        .map_err(|_| Error::StoreIoError("Failed to write".to_string()))?;
                }
            }
        }

        FileType::SingleFile => {
            let filename = &version
                .files
                .first()
                .expect("Single file should contain one full glob filename")
                .glob;

            fs::write(version_dir.join(filename), file)
                .map_err(|_| Error::StoreIoError("Failed to write single file".to_string()))?;
        }
    }
    let config = FontConfig {
        style_preference,
        format_preference,
    };
    let config = toml::to_string_pretty(&config)?;
    fs::write(font_dir.join("font.toml"), config)
        .map_err(|_| Error::StoreIoError("Failed to write font config file".to_string()))?;
    Ok(version_dir)
}

pub(crate) fn list_fonts() -> Result<Vec<StoredFont>, Error> {
    let mut installed_fonts: Vec<StoredFont> = Vec::new();
    let repos = fs::read_dir(STORE_DIR.as_path())
        .map_err(|_| Error::StoreIoError("Failed to read store directory".to_string()))?;
    for repo in repos {
        let repo =
            repo.map_err(|_| Error::StoreIoError("Failed to read repo directory".to_string()))?;
        let fonts = fs::read_dir(repo.path())
            .map_err(|_| Error::StoreIoError("Failed to read repo directory".to_string()))?;
        for font in fonts {
            let font = font
                .map_err(|_| Error::StoreIoError("Failed to read version directory".to_string()))?;
            let versions = fs::read_dir(font.path())
                .map_err(|_| Error::StoreIoError("Failed to read font directory".to_string()))?;
            for version in versions {
                let version = version.map_err(|_| {
                    Error::StoreIoError("Failed to read version directory".to_string())
                })?;
                if version.path().is_file() {
                    continue;
                }
                installed_fonts.push(StoredFont {
                    id: font.file_name().to_string_lossy().to_string(),
                    repo_id: repo.file_name().to_string_lossy().to_string(),
                    version: version.file_name().to_string_lossy().to_string(),
                });
            }
        }
    }
    Ok(installed_fonts)
}

pub(crate) fn find_fonts_in_repo(
    font_id: Option<&str>,
    repo_id: &str,
) -> Result<Vec<StoredFont>, Error> {
    let mut matches: Vec<StoredFont> = Vec::new();
    let dir = STORE_DIR.join(repo_id);
    let fonts = fs::read_dir(dir)
        .map_err(|_| Error::StoreIoError("Failed to read font directory".to_string()))?;
    for font in fonts {
        let font =
            font.map_err(|_| Error::StoreIoError("Failed to read version directory".to_string()))?;
        if font_id.is_none() || font.file_name().to_string_lossy() == font_id.unwrap() {
            let versions = fs::read_dir(font.path())
                .map_err(|_| Error::StoreIoError("Failed to read font directory".to_string()))?;
            for version in versions {
                let version = version.map_err(|_| {
                    Error::StoreIoError("Failed to read version directory".to_string())
                })?;
                if version.path().is_file() {
                    continue;
                }
                matches.push(StoredFont {
                    id: font.file_name().to_string_lossy().to_string(),
                    repo_id: repo_id.to_string(),
                    version: version.file_name().to_string_lossy().to_string(),
                });
            }
        }
    }
    Ok(matches)
}

pub(crate) fn find_font(font_id: &str) -> Result<Vec<StoredFont>, Error> {
    let mut matches: Vec<StoredFont> = Vec::new();
    let repos = fs::read_dir(STORE_DIR.as_path())
        .map_err(|_| Error::StoreIoError("Failed to read store directory".to_string()))?;
    for repo in repos {
        find_fonts_in_repo(
            Some(font_id),
            repo.map_err(|_| Error::StoreIoError("Failed to read repo directory".to_string()))?
                .file_name()
                .to_string_lossy()
                .as_ref(),
        )
        .map(|mut fonts| matches.append(&mut fonts))
        .ok();
    }
    Ok(matches)
}

pub(crate) fn remove_font(font_id: &str, repo_id: &str) -> Result<PathBuf, Error> {
    let font_dir = STORE_DIR.join(repo_id).join(font_id);
    fs::remove_dir_all(&font_dir)
        .map_err(|_| Error::StoreIoError("Failed to remove font directory".to_string()))?;
    Ok(font_dir)
}

pub(crate) fn get_font_preference(font_id: &str, repo_id: &str) -> FontConfig {
    let font_config_path = STORE_DIR.join(repo_id).join(font_id).join("font.toml");
    fs::read_to_string(font_config_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default()
}
