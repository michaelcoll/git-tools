/*
 * Copyright (c) 2023 Michaël COLL.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
                        } else if fs.ratio < 0_f32 {
                            println!(
                                "{} {} ({})",
                                " — Lost".bright_black(),
                                format!("{:.2}%", fs.ratio).red().bold(),
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
