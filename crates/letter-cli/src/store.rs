use std::{
    fs,
    io::{self, Cursor},
    path::PathBuf,
    sync::LazyLock,
};

use directories::ProjectDirs;
use globset::{Glob, GlobSetBuilder};
use letter_core::font::{FileType, Version};
use zip::ZipArchive;

pub(crate) static STORE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    ProjectDirs::from("", "", "letter")
        .unwrap()
        .data_dir()
        .join("store")
});

pub(crate) fn store_font(
    version: &Version,
    font_id: &str,
    repo_id: &str,
    file: Vec<u8>,
) -> Result<PathBuf, String> {
    let font_dir = STORE_DIR.join(repo_id).join(font_id).join(&version.number);
    fs::create_dir_all(&font_dir).expect("Failed to create font directory");

    match version.r#type {
        FileType::Zip => {
            let cursor = Cursor::new(file);
            let mut zip = ZipArchive::new(cursor).expect("Can't open zip");

            let mut glob_set_builder = GlobSetBuilder::new();
            for pattern in &version.files {
                glob_set_builder.add(Glob::new(&pattern.glob).expect("Failed to create glob"));
            }
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
