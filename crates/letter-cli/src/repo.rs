use clap::{Parser, ValueHint};
use crate::config::{read_config, write_config};
use url::Url;
use crate::config_file::{Config, Repo};

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
}

#[derive(Parser, Debug)]
pub(crate) struct RemoveArgs {
    id: String,
}

pub(crate) fn run(cmd: Command) {
    match cmd {
        Command::List => list_repos(),
        Command::Add(args) => add_repo(args),
        _ => todo!(),
    }
}


fn list_repos() {
    println!("List of repositories:");
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

    for repo in &config.repo {
        if url == repo.url {
            println!("Repo {} already exists", repo.id);
            return;
        }
    }

    config.repo.push(Repo {
        id: "ofa".to_string(),
        url,
    });

    match write_config(&config) {
        Ok(_) => println!("Repo added"),
        Err(err) => {
            println!("{}", err);
        }
    };
}
