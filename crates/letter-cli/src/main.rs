mod args;
mod config;
mod config_file;
mod network;
mod os;
mod repo;
mod repos;
mod store;

use clap::{CommandFactory, Parser};
use clap_complete::generate;

use crate::repos::RepoList;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Cli {
    /// Install a font
    Install(args::InstallFontArgs),
    /// Remove a font
    Remove(args::FontArgs),
    /// List installed fonts
    List,
    /// Update fonts
    Update(args::UpdateFontArgs),
    /// Enable an installed font
    Enable(args::FontArgs),
    /// Disable an installed font
    Disable(args::FontArgs),
    /// Search fonts in repositories
    Search(args::SearchArgs),

    /// Manage repositories
    #[clap(subcommand)]
    Repo(repo::Command),

    /// Generate shell completions
    Completions(args::CompletionArgs),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let repo_list = RepoList::new();
    match args {
        Cli::Install(args) => repo_list.install_font_cmd(args).await,
        Cli::Remove(args) => repo_list.remove_font_cmd(args).await,
        Cli::List => repo_list.list_fonts_cmd().await,
        Cli::Update(args) => repo_list.update_fonts_cmd(args).await,
        Cli::Enable(args) => repo_list.enable_font_cmd(args).await,
        Cli::Disable(args) => repo_list.disable_font_cmd(args).await,
        Cli::Search(args) => repo_list.search_fonts_cmd(args).await,
        Cli::Repo(cmd) => repo::run(cmd).await,

        Cli::Completions(args) => {
            generate(
                args.shell,
                &mut Cli::command(),
                "letter",
                &mut std::io::stdout(),
            );
        }
    }
}
