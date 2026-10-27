use clap::{Parser, ValueEnum};
use letter_core::font::FontFormat;

#[derive(Parser, Debug)]
pub(crate) struct FontArgs {
    pub(crate) id: String,

    #[arg(short, long)]
    pub(crate) repo: Option<String>,
}

#[derive(Parser, Debug)]
pub(crate) struct InstallFontArgs {
    pub(crate) id: String,
    #[arg(short, long)]
    pub(crate) repo: Option<String>,

    /// Install specific version of the font. If not specified, the latest version will be installed
    #[arg(short, long)]
    pub(crate) version: Option<String>,

    /// Install the variable variant of the font if available. Can't be used together with --static
    #[arg(short('V'), long)]
    pub(crate) variable: bool,

    /// Install the static variant of the font if available. Can't be used together with --variable
    #[arg(short('S'), long)]
    pub(crate) r#static: bool,

    /// Request a specific format of the font.
    #[arg(short, long, value_enum)]
    pub(crate) format: Option<CliFontFormat>,
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum CliFontFormat {
    OTF,
    TTF,
    WOFF,
    WOFF2,
    TTC,
}

impl CliFontFormat {
    pub(crate) fn to_font_format(&self) -> FontFormat {
        match self {
            CliFontFormat::OTF => FontFormat::OTF,
            CliFontFormat::TTF => FontFormat::TTF,
            CliFontFormat::WOFF => FontFormat::WOFF,
            CliFontFormat::WOFF2 => FontFormat::WOFF2,
            CliFontFormat::TTC => FontFormat::TTC,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct StoredFont {
    pub id: String,
    pub repo_id: String,
    pub version: String,
}

#[derive(Clone, Debug)]
pub(crate) struct InstalledFont {
    pub id: String,
    pub repo_id: String,
}
