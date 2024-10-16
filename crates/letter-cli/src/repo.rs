use clap::{Parser, ValueHint};
use crate::config::{read_config, write_config};
use url::Url;
use crate::config_file::{Config, Repo};
use ansi_term::Style;

#[derive(Parser, Debug)]
pub(crate) enum Command {
    Add(AddArgs),
    Remove(RemoveArgs),
    List,
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

pub(crate) fn run(cmd: Command) {
    match cmd {
        Command::List => list_repos(),
        Command::Add(args) => add_repo(args),
        Command::Remove(args) => remove_repo(args),
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

fn add_repo(args: AddArgs) {
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

    config.repo.push(Repo {
        id,
        url,
    });

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
        Ok(_) => println!("Repo added"),
        Err(err) => {
            println!("{}", err);
        }
    };
    println!("Repo {} was removed", args.id);
}
