use std::{
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::font::InstalledFont;

pub(crate) fn install_font(font_id: &str, repo_id: &str, store_path: PathBuf) {
    cfg_select! {
        target_os = "linux" => {
            install_linux(font_id, repo_id, store_path)
        }
    }
}

pub(crate) fn list_fonts() -> Vec<InstalledFont> {
    let mut installed_fonts: Vec<InstalledFont> = Vec::new();
    let repos = fs::read_dir(LINUX_FONT_DIR.as_path()).expect("Failed to read font directory");
    for repo in repos {
        let repo = repo.expect("Failed to read repo directory");
        let fonts = fs::read_dir(repo.path()).expect("Failed to read repo directory");
        for font in fonts {
            let font = font.expect("Failed to read font directory");
            installed_fonts.push(InstalledFont {
                id: font.file_name().to_string_lossy().to_string(),
                repo_id: repo.file_name().to_string_lossy().to_string(),
            });
        }
    }
    installed_fonts
}

#[cfg(target_os = "linux")]
static LINUX_FONT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(&std::env::home_dir().expect("Failed loading home dir"))
        .join(".local/share/fonts")
        .join("letter")
});

#[cfg(target_os = "linux")]
fn refresh_cache_linux() -> Vec<u8> {
    use std::process::Command;

    Command::new("fc-cache")
        .arg("--verbose")
        .output()
        .expect("Failed to refresh cache")
        .stdout
}

#[cfg(target_os = "linux")]
fn install_linux(font_id: &str, repo_id: &str, store_path: PathBuf) {
    use std::os::unix;

    let repo_path = LINUX_FONT_DIR.join(repo_id);
    fs::create_dir_all(&repo_path).expect("Failed to create dir");
    unix::fs::symlink(store_path, repo_path.join(font_id)).expect("Failed to install");
    let out = refresh_cache_linux();
    let out = String::from_utf8(out).unwrap();
    println!("{}", out)
}

#[cfg(target_os = "linux")]
fn remove_linux(font_id: &str, repo_id: &str) {
    let font_path = LINUX_FONT_DIR.join(repo_id).join(font_id);
    fs::remove_file(font_path).expect("Failed to remove font");
    refresh_cache_linux();
}

pub(crate) fn remove_font(font_id: &str, repo_id: &str) {
    cfg_select! {
        target_os = "linux" => {
            remove_linux(font_id, repo_id)
        }
    }
}
