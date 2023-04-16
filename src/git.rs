use colored::Colorize;
use std::process::Command;

use size::Size;

#[derive(Debug, PartialEq)]
pub struct FileStat {
    pub diff: Size,
    pub ratio: f32,
}

fn get_size_of_dir(folder: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let output = Command::new("du").current_dir(folder).arg("-s").output()?;

    let size = String::from_utf8(output.stdout)?
        .lines()
        .take(1)
        .next()
        .unwrap()
        .split('\t')
        .next()
        .map(String::from)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap();

    Ok(size)
}

pub fn compact_repo(folder: String) -> Result<FileStat, Box<dyn std::error::Error>> {
    let size_before = get_size_of_dir(&folder)?;

    let output = Command::new("git")
        .current_dir(&folder)
        .arg("gc")
        .arg("--aggressive")
        .output()?;

    if !output.status.success() {
        println!(
            "{} Command executed with failing error code",
            "Warn :".yellow()
        );
    }

    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .for_each(|x| println!("{:?}", x));

    let size_after = get_size_of_dir(&folder)?;

    Ok(file_stat(size_before, size_after))
}

fn file_stat(size_before: u64, size_after: u64) -> FileStat {
    let diff = size_before - size_after;

    let ratio = diff as f32 / size_before as f32;

    FileStat {
        diff: Size::from_bytes(diff),
        ratio: (ratio * 10000_f32).round() / 100_f32,
    }
}

#[cfg(test)]
mod tests {
    use size::Size;

    use crate::git::{file_stat, FileStat};

    #[test]
    fn test_ratio() {
        assert_eq!(
            file_stat(1024, 768),
            FileStat {
                diff: Size::from_bytes(256),
                ratio: 25.0,
            }
        );
    }

    #[test]
    fn test_ratio_rounded() {
        assert_eq!(
            file_stat(1024, 900),
            FileStat {
                diff: Size::from_bytes(124),
                ratio: 12.11,
            }
        );
    }
}
