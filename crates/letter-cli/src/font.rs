use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct FontArgs {
    pub(crate) id: String,
}
