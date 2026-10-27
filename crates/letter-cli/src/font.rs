use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct FontArgs {
    pub(crate) id: String,
}

pub(crate) struct InstalledFont {
    pub id: String,
    pub repo_id: String,
    pub version: String,
}
