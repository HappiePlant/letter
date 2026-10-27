use std::{
    fs,
    io::{self, Cursor},
    path::PathBuf,
    sync::LazyLock,
};

use directories::ProjectDirs;
use globset::{Glob, GlobSetBuilder};
use letter_core::font::{FileType, Version, VersionFile};
use zip::ZipArchive;

#[derive(Clone, Debug)]
pub(crate) struct StoredFont {
    pub id: String,
    pub repo_id: String,
    pub version: String,
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
) -> Result<PathBuf, String> {
    let font_dir = STORE_DIR.join(repo_id).join(font_id).join(&version.number);
    fs::create_dir_all(&font_dir).expect("Failed to create font directory");

    match version.r#type {
        FileType::Zip => {
            let cursor = Cursor::new(file);
            let mut zip = ZipArchive::new(cursor).expect("Can't open zip");

            let mut glob_set_builder = GlobSetBuilder::new();
            glob_set_builder.add(Glob::new(&version_file.glob).expect("Failed to create glob"));
            let glob_set = glob_set_builder.build().unwrap();

            for i in 0..zip.len() {
                let mut item = zip.by_index(i).unwrap();
                let path = item.enclosed_name().expect("Failed to read path");
                if glob_set.is_match(&path) {
                    let mut out = fs::File::create(font_dir.join(path.file_name().unwrap()))
                        .expect("Can't create file");
                    io::copy(&mut item, &mut out).expect("Failed to write");
                }
            }
        }

        FileType::SingleFile => {
            let filename = &version
                .files
                .first()
                .expect("Single file should contain one full glob filename")
                .glob;

            fs::write(font_dir.join(filename), file).expect("Failed to write file");
        }
    }
    return Ok(font_dir);
}

pub(crate) fn list_fonts() -> Vec<StoredFont> {
    let mut installed_fonts: Vec<StoredFont> = Vec::new();
    let repos = fs::read_dir(STORE_DIR.as_path()).expect("Failed to read store directory");
    for repo in repos {
        let repo = repo.expect("Failed to read repo directory");
        let fonts = fs::read_dir(repo.path()).expect("Failed to read repo directory");
        for font in fonts {
            let font = font.expect("Failed to read version directory");
            let versions = fs::read_dir(font.path()).expect("Failed to read font directory");
            for version in versions {
                let version = version.expect("Failed to read version directory");
                installed_fonts.push(StoredFont {
                    id: font.file_name().to_string_lossy().to_string(),
                    repo_id: repo.file_name().to_string_lossy().to_string(),
                    version: version.file_name().to_string_lossy().to_string(),
                });
            }
        }
    }
    installed_fonts
}

pub(crate) fn find_font(font_id: &str) -> Result<Vec<StoredFont>, ()> {
    let mut matches: Vec<StoredFont> = Vec::new();
    let repos = fs::read_dir(STORE_DIR.as_path()).expect("Failed to read store directory");
    for repo in repos {
        let repo = repo.expect("Failed to read repo directory");
        let fonts = fs::read_dir(repo.path()).expect("Failed to read repo directory");
        for font in fonts {
            let font = font.expect("Failed to read version directory");
            if font.file_name().to_string_lossy() == font_id {
                let versions = fs::read_dir(font.path()).expect("Failed to read font directory");
                for version in versions {
                    let version = version.expect("Failed to read version directory");
                    matches.push(StoredFont {
                        id: font.file_name().to_string_lossy().to_string(),
                        repo_id: repo.file_name().to_string_lossy().to_string(),
                        version: version.file_name().to_string_lossy().to_string(),
                    });
                }
            }
        }
    }
    Ok(matches)
}

pub(crate) fn remove_font(font_id: &str, repo_id: &str) -> Result<PathBuf, ()> {
    let font_dir = STORE_DIR.join(repo_id).join(font_id);
    if font_dir.exists() {
        fs::remove_dir_all(&font_dir).expect("Failed to remove font directory");
        Ok(font_dir)
    } else {
        Err(())
    }
}
