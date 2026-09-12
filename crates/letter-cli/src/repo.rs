use crate::config_file::{Config, Repo};
use crate::{
    config::{read_config, write_config},
    network::RepoClient,
};
use ansi_term::Style;
use clap::{Parser, ValueHint};
use url::Url;

#[derive(Parser, Debug)]
pub(crate) enum Command {
    Add(AddArgs),
    Remove(RemoveArgs),
    List,
    Info(InfoArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct AddArgs {
    #[clap(value_hint = ValueHint::Url)]
    url: String,

    #[arg(long)]
    id: Option<String>,
}

#[derive(Parser, Debug)]
pub(crate) struct RemoveArgs {
    id: String,
}

#[derive(Parser, Debug)]
pub(crate) struct InfoArgs {
    id: String,
}

pub(crate) async fn run(cmd: Command) {
    match cmd {
        Command::List => list_repos(),
        Command::Add(args) => add_repo(args).await,
        Command::Remove(args) => remove_repo(args),
        Command::Info(args) => get_repo_info(args),
    }
}

fn list_repos() {
    let config: Config = match read_config() {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!("{}", Style::new().bold().paint("Repo ID\t\tURL"));
    for repo in config.repo {
        println!("{}\t\t{}", repo.id, repo.url);
    }
    // TODO: Make tab size dynamic
}

fn get_repo_info(args: InfoArgs) {}

async fn add_repo(args: AddArgs) {
    let mut config: Config = match read_config() {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let url = match Url::parse(&args.url) {
        Ok(url) => url,
        Err(_) => {
            println!("Invalid URL");
            return;
        }
    };

    let client = RepoClient {
        base_url: url.to_string(),
    };
    let repo_info = client.get_repo_info().await.unwrap();

    println!("{}{}", Style::new().bold().paint("Name: "), repo_info.name);
    println!(
        "{}{}",
        Style::new().bold().paint("Description: "),
        repo_info.description
    );
    println!("{}{}", Style::new().bold().paint("URL: "), url);
    // TODO: Ask confirmation about ID or allow entering a custom ID
    println!(
        "{}{}{}",
        Style::new().bold().paint("Installing with proposed ID \""),
        repo_info.id,
        Style::new().bold().paint("\"...")
    );

    let id = args.id.unwrap_or_else(|| "ofa".to_string());

    for repo in &config.repo {
        if url == repo.url {
            println!("Repo {} already exists (same url)", repo.id);
            return;
        }
        if id == repo.id {
            println!("Repo with name {} already exists", id);
            return;
        }
    }

    config.repo.push(Repo { id, url });

    match write_config(&config) {
        Ok(_) => println!("Repo added"),
        Err(err) => {
            println!("{}", err);
        }
    };
}

fn remove_repo(args: RemoveArgs) {
    let mut config: Config = match read_config() {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let index = config.repo.iter().position(|repo| repo.id == args.id);

    if index.is_none() {
        println!("Repo {} was not found", args.id);
        return;
    }

    config.repo.remove(index.unwrap());

    match write_config(&config) {
        Ok(_) => println!("Repo {} was removed", args.id),
        Err(err) => {
            println!("{}", err);
        }
    };
}
