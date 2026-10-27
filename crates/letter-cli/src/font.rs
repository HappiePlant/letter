use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct FontArgs {
    pub(crate) id: String,
    #[arg(short, long)]
    pub(crate) repo: Option<String>,
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
