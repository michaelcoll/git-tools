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
