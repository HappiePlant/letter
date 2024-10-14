use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct FontArgs {
    id: String,
}

pub(crate) fn install_font(args: FontArgs) {}