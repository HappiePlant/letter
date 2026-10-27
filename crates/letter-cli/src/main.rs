mod config;
mod config_file;
mod font;
mod network;
mod os;
mod repo;
mod repos;
mod store;

use clap::Parser;

use crate::repos::RepoList;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Cli {
    Install(font::InstallFontArgs),
    Remove(font::FontArgs),
    Disable(font::FontArgs),
    Enable(font::FontArgs),
    List,

    #[clap(subcommand)]
    Repo(repo::Command),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
    let repo_list = RepoList::new();
    match args {
        Cli::Repo(cmd) => repo::run(cmd).await,
        Cli::Install(args) => repo_list.install_font_cmd(args).await,
        Cli::List => repo_list.list_fonts_cmd().await,
        Cli::Remove(args) => repo_list.remove_font_cmd(args).await,
        Cli::Enable(args) => repo_list.enable_font_cmd(args).await,
        Cli::Disable(args) => repo_list.disable_font_cmd(args).await,
    }
}
