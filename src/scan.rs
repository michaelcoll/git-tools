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

use colored::Colorize;
use std::fs;
use std::io::Error;

pub fn scan(folder: String) -> Result<Vec<String>, Error> {
    let mut paths = Vec::new();

    for path in fs::read_dir(&folder)? {
        let path = path?;
        let is_dir = path.file_type()?.is_dir();
        let filename = path.file_name();

        if is_dir && filename == ".git" {
            paths.push(folder.clone())
        } else if is_dir {
            match scan(path.path().display().to_string()) {
                Ok(sub_paths) => {
                    paths = [paths, sub_paths].concat();
                }
                Err(e) => {
                    println!(
                        "{} Can't read {}, {}",
                        "Warn:".yellow(),
                        path.path().display(),
                        e
                    );
                }
            }
        }
    }

    Ok(paths)
}
