use clap::Parser;

use crate::commands::Commands;

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
