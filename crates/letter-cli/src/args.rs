use clap::{Parser, ValueEnum};
use clap_complete::Shell;
use letter_core::font::FontFormat;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub(crate) struct FontArgs {
    pub(crate) id: String,

    /// Repository id
    #[arg(short, long)]
    pub(crate) repo: Option<String>,
}

#[derive(Parser, Debug)]
pub(crate) struct SearchArgs {
    /// Optional search query. If not specified, all fonts will be returned
    pub(crate) query: Option<String>,

    /// Only search in this repository
    #[arg(short, long)]
    pub(crate) repo: Option<String>,
}

#[derive(Parser, Debug)]
pub(crate) struct InstallFontArgs {
    pub(crate) id: String,

    /// Repository id
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

#[derive(Parser, Debug)]
pub(crate) struct UpdateFontArgs {
    /// Font id, if not specified, all installed fonts will be updated
    pub(crate) id: Option<String>,

    /// Repository id, will only update fonts from this repository if specified
    #[arg(short, long)]
    pub(crate) repo: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Parser, Debug)]
pub(crate) struct CompletionArgs {
    /// Shell to generate completion for
    #[arg(value_parser = clap::value_parser!(Shell))]
    pub(crate) shell: Shell,
}
