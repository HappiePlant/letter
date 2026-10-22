use std::{fs, path::PathBuf, sync::LazyLock};

use directories::ProjectDirs;
use letter_core::font::{FileType, Version};

static STORE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    ProjectDirs::from("", "", "letter")
        .unwrap()
        .data_dir()
        .join("store")
});

pub(crate) fn install_font_files(version: &Version, font_id: &str, repo_id: &str, file: Vec<u8>) {
    let font_dir = STORE_DIR.join(repo_id).join(font_id).join(&version.number);
    fs::create_dir_all(&font_dir).expect("Failed to create font directory");

    match version.r#type {
        FileType::Zip => {}
        FileType::SingleFile => {
            let filename = &version
                .files
                .first()
                .expect("Single file should contain one full glob filename")
                .glob;
            fs::write(font_dir.join(filename), file).expect("Failed to write file");
        }
    }
}
