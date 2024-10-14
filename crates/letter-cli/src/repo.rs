use clap::{Parser, ValueHint};
use url::Url;

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

fn add_repo(args: AddArgs) {}