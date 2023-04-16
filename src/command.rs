use crate::git::compact_repo;
use crate::scan::scan;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::Path;

#[derive(Parser)]
#[clap(
    author = "Michaël COLL",
    version,
    about = "Git TooLs",
    long_about = None
)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Garbage collect
    Gc {
        /// The folder to garbage collect.
        /// Git TooLs will search recursively git repository in this folder.
        /// By default the current folder is used.
        #[clap(short = 'f', long, value_parser)]
        folder: Option<String>,
    },
}

pub fn gc(folder: String) {
    if !Path::new(&folder).is_dir() {
        eprintln!("{} folder {} does not exists !", "Error:".red(), folder);
        std::process::exit(exitcode::USAGE);
    }

    match scan(folder) {
        Ok(git_repos) => {
            for path in git_repos {
                print!("{path}");

                match compact_repo(path) {
                    Ok(fs) => {
                        if fs.ratio > 0_f32 {
                            println!(
                                "{} {} ({})",
                                " — Saved".bright_black(),
                                format!("{:.2}%", fs.ratio).green().bold(),
                                fs.diff.to_string().bright_black()
                            )
                        } else {
                            println!("{}", " — No change".bright_black())
                        }
                    }
                    Err(e) => {
                        println!("{} {}", "Warn :".yellow(), e)
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red(), e);
            std::process::exit(exitcode::USAGE);
        }
    }
}
