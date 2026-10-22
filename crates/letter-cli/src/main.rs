mod config;
mod config_file;
mod os;
mod font;
mod network;
mod repo;
mod repos;
mod store;

use clap::Parser;

use crate::repos::RepoList;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Cli {
    Install(font::FontArgs),
    Remove(font::FontArgs),
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
        _ => todo!(),
    }
}
