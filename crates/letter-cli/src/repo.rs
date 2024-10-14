use clap::{Parser};

#[derive(Parser, Debug)]
pub(crate) enum Command {
    Add(RepoArgs),
    Remove(RepoArgs),
    List,
}

#[derive(Parser, Debug)]
pub(crate) struct RepoArgs {
    id: String,
}

pub(crate) fn run(cmd: Command) {
    match cmd {
        Command::List => list_repos(),
        _ => todo!(),
    }
}

fn list_repos() {
    println!("List of repositories:");
}