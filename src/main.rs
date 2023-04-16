use std::env;

use clap::Parser;

use crate::command::{Cli, Commands};

mod command;
mod git;
mod scan;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gc { folder } => {
            let current_dir = env::current_dir()?;

            match folder {
                None => command::gc(current_dir.display().to_string()),
                Some(path) => command::gc(path.to_string()),
            }
        }
    }

    Ok(())
}
