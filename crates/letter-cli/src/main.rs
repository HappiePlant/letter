mod repo;
mod font;

use clap::Parser;

#[derive(Parser, Debug)]
enum Cli {
    Install(font::FontArgs),
    Remove(font::FontArgs),
    List,

    #[clap(subcommand)]
    Repo(repo::Command),
}


fn main() {
    let args = Cli::parse();
    println!("Hello, world!");
    println!("{:?}", args);
    match args {
        Cli::Repo(cmd) => repo::run(cmd),
        _ => todo!(),
    }
}
