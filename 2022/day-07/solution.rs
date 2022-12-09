#!/usr/bin/env rust-script

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const INPUT_FILE: &str = "input";

#[derive(Debug, Default)]
struct Entry {
    name: String,
    type_: EntryType,
}

impl Entry {
    fn new(line: &str) -> Option<Self> {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.first()?.starts_with("dir") {
            Some(Self {
                name: parts.get(1)?.to_string(),
                type_: EntryType::Directory,
            })
        } else {
            Some(Self {
                name: parts.get(1)?.to_string(),
                type_: EntryType::File(parts.first().and_then(|v| v.parse().ok())?),
            })
        }
    }
}

#[derive(Debug)]
enum EntryType {
    Directory,
    File(u32),
    Unknown,
}

impl Default for EntryType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Default)]
struct Directory {
    name: String,
    entries: Vec<Entry>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    fn calculate_size(&self, directories: &[Directory]) -> u32 {
        self.entries.iter().fold(0, |mut sum, entry| {
            match entry.type_ {
                EntryType::Directory => {
                    sum += directories
                        .iter()
                        .find(|dir| dir.name == entry.name)
                        .unwrap()
                        .calculate_size(directories)
                }
                EntryType::File(size) => sum += size,
                EntryType::Unknown => {}
            }
            sum
        })
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    List,
    Unknown,
}

impl Default for Command {
    fn default() -> Self {
        Self::Unknown
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_parts = s
            .trim_start_matches('$')
            .split_whitespace()
            .collect::<Vec<&str>>();
        match command_parts.first() {
            Some(&"cd") => Ok(Self::ChangeDirectory(
                command_parts.get(1).ok_or(())?.to_string(),
            )),
            Some(&"ls") => Ok(Self::List),
            _ => Ok(Self::Unknown),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut directories = Vec::<Directory>::new();
    let mut current_directory = Directory::default();
    let mut current_level = PathBuf::new();
    let mut current_command = Command::Unknown;
    for line in input.lines() {
        if line.starts_with('$') {
            current_command = Command::from_str(line).unwrap();
        }
        match current_command {
            Command::ChangeDirectory(ref name) => {
                if name == ".." {
                    current_level.pop();
                    continue;
                }
                if !current_directory.name.is_empty() {
                    directories.push(current_directory);
                }
                current_level.push(name);
                current_directory = Directory::new(&current_level.to_string_lossy());
            }
            Command::List => {
                if line.starts_with('$') {
                    continue;
                }
                let mut entry = Entry::new(line).unwrap();
                entry.name = current_level.join(entry.name).to_string_lossy().to_string();
                current_directory.entries.push(entry);
            }
            Command::Unknown => {
                eprintln!("Unknown command: {line}");
            }
        }
    }
    directories.push(current_directory);
    let mut total_sizes = Vec::new();
    for directory in directories.iter() {
        let total_size = directory.calculate_size(&directories);
        total_sizes.push(total_size);
    }
    total_sizes.sort();

    println!(
        "Part1: {}",
        total_sizes.iter().filter(|v| v < &&100000).sum::<u32>()
    );

    println!(
        "Part2: {}",
        total_sizes
            .iter()
            .find(|v| *v + 70000000 - total_sizes[total_sizes.len() - 1] > 30000000)
            .unwrap()
    );

    Ok(())
}
