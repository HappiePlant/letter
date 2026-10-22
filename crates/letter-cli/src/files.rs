use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use letter_core::font::Version;

static FONT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| Path::new("~/.local/share/fonts").join("letter"));
