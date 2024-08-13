mod cli;
mod commands;
mod models;
mod podman;
mod utils;

use anyhow::Result;

use cli::get_cli;

fn main() -> Result<()> {
    let cli = get_cli();
    cli.commands.exec()?;
    Ok(())
}
