use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use letter_core::font::Version;

static FONT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| Path::new("~/.local/share/fonts").join("letter"));

pub(crate) fn install_font_files(version: &Version, font_id: &str, repo_id: &str, file: Vec<u8>) {
    let font_dir = FONT_DIR
        .join(repo_id)
        .join(font_id)
        .join(&version.number);
    std::fs::create_dir_all(&font_dir).expect("Failed to create font directory");
    let file_path = font_dir.join(format!("{}.ttf", font_id));
    std::fs::write(file_path, file).expect("Failed to write font file");
}
