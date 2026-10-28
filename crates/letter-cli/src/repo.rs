use crate::config_file::{Config, Repo};
use crate::error::Error;
use crate::{
    config::{read_config, write_config},
    network::RepoClient,
};
use ansi_term::Style;
use clap::{Parser, ValueHint};
use letter_core::RepoInfo;
use url::Url;

#[derive(Parser, Debug)]
pub(crate) enum Command {
    /// Add a repository
    Add(AddArgs),
    /// Remove a repository
    Remove(RemoveArgs),
    /// List added repositories
    List,
    /// Fetch info of a repository
    Info(InfoArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct AddArgs {
    /// URL of the repository to add
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

pub(crate) async fn run(cmd: Command) -> Result<(), Error> {
    match cmd {
        Command::List => list_repos(),
        Command::Add(args) => add_repo(args).await,
        Command::Remove(args) => remove_repo(args),
        Command::Info(args) => get_repo_info(args).await,
    }
}

fn list_repos() -> Result<(), Error> {
    let config: Config = read_config()?;

    println!("{}", Style::new().bold().paint("Repo ID\t\tURL"));
    for repo in config.repo {
        println!("{}\t\t{}", repo.id, repo.url);
    }
    Ok(())
    // TODO: Make tab size dynamic
}

async fn get_repo_info(args: InfoArgs) -> Result<(), Error> {
    let config: Config = read_config()?;
    let client = RepoClient {
        base_url: config
            .repo
            .iter()
            .find(|repo| repo.id == args.id)
            .ok_or(Error::RepoNotFound(args.id))?
            .url
            .clone()
            .to_string(),
    };
    let repo_info = client.get_repo_info().await?;
    print_repo_info(&repo_info);
    Ok(())
}

fn print_repo_info(info: &RepoInfo) {
    println!("{}{}", Style::new().bold().paint("Name: "), info.name);
    println!(
        "{}{}",
        Style::new().bold().paint("Description: "),
        info.description
    );
}

async fn add_repo(args: AddArgs) -> Result<(), Error> {
    let mut config: Config = read_config()?;

    let url = Url::parse(&args.url).map_err(|_| Error::ArgsInvalidUrl)?;

    let client = RepoClient {
        base_url: url.to_string(),
    };
    let repo_info = client.get_repo_info().await?;

    print_repo_info(&repo_info);

    println!("{}{}", Style::new().bold().paint("URL: "), url);
    // TODO: Ask confirmation about ID or allow entering a custom ID
    println!(
        "{}{}{}",
        Style::new().bold().paint("Installing with proposed ID \""),
        repo_info.id,
        Style::new().bold().paint("\"...")
    );

    let id = args.id.unwrap_or_else(|| repo_info.id);

    for repo in &config.repo {
        if url == repo.url {
            return Err(Error::RepoUrlAlreadyExists);
        }
        if id == repo.id {
            return Err(Error::RepoIdAlreadyExists);
        }
    }

    config.repo.push(Repo { id, url });

    write_config(&config)?;
    println!("Repo added");
    Ok(())
}

fn remove_repo(args: RemoveArgs) -> Result<(), Error> {
    let mut config: Config = read_config()?;

    let index = config.repo.iter().position(|repo| repo.id == args.id);

    match index {
        Some(i) => {
            config.repo.remove(i);
        }
        None => {
            return Err(Error::RepoNotFound(args.id));
        }
    }

    write_config(&config)?;
    println!("Repo {} was removed", args.id);
    Ok(())
}
