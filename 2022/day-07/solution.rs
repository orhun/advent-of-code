#!/usr/bin/env rust-script

use std::error::Error;
use std::fmt::Write;
use std::fs::{self, File};
use std::io::Write as IoWrite;
use std::path::PathBuf;
use std::str::FromStr;

const INPUT_FILE: &str = "input";

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    List,
    Unknown,
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

#[derive(Debug)]
struct Entry {
    name: String,
    type_: EntryType,
}

impl FromStr for Entry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        if parts.first().ok_or(())?.starts_with("dir") {
            Ok(Self {
                name: parts.get(1).ok_or(())?.to_string(),
                type_: EntryType::Directory,
            })
        } else {
            Ok(Self {
                name: parts.get(1).ok_or(())?.to_string(),
                type_: EntryType::File(parts.first().and_then(|v| v.parse().ok()).ok_or(())?),
            })
        }
    }
}

#[derive(Clone, Debug)]
enum EntryType {
    Directory,
    File(u32),
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let mut script = String::new();
    writeln!(script, "#!/usr/bin/env bash")?;
    let mut current_command = Command::Unknown;
    for line in input.lines() {
        if line.starts_with('$') {
            current_command = Command::from_str(line).expect("failed to parse command");
        }
        match current_command {
            Command::ChangeDirectory(ref name) => {
                if name == "/" {
                    writeln!(script, "{}", r#"tempdir="$(mktemp -d)""#)?;
                    writeln!(script, "{}", r#"cd "$tempdir""#)?;
                } else {
                    writeln!(script, "cd {name}")?;
                }
            }
            Command::List => {
                if line.starts_with('$') {
                    continue;
                }
                let entry = Entry::from_str(line).expect("failed to parse entry");
                match entry.type_ {
                    EntryType::Directory => {
                        writeln!(script, "mkdir {}", entry.name)?;
                    }
                    EntryType::File(size) => {
                        writeln!(script, "echo {} > {}", size, entry.name)?;
                    }
                }
            }
            Command::Unknown => {
                eprintln!("Unknown command: {line}");
            }
        }
    }
    let mut script1 = script.clone();
    let mut script2 = script.clone();
    writeln!(
        script1,
        "{}",
        r#"
cd "$tempdir"
result=0
readarray -t dirs <<<"$(find . -type d)"
for dir in "${dirs[@]}"; do
	readarray -t files <<<"$(find $dir -type f)"
	output=$(awk '{arr[FNR]+=$1}END{for(i=1;i<=FNR;i+=1){print arr[i]}}' "${files[@]}")
	if [ "$output" -lt 100000 ]; then
		result=$((result + output))
	fi
done
echo "$result""#
    )?;
    writeln!(
        script2,
        "{}",
        r#"
cd "$tempdir"
result=()
readarray -t dirs <<<"$(find . -type d)"
for dir in "${dirs[@]}"; do
	readarray -t files <<<"$(find $dir -type f)"
	output=$(awk '{arr[FNR]+=$1}END{for(i=1;i<=FNR;i+=1){print arr[i]}}' "${files[@]}")
	result+=("$output")
done
IFS=$'\n'
total=$(echo "${result[*]}" | sort -nr | head -n1)
readarray -t sorted_result <<<"$(echo "${result[*]}" | sort -n)"
for v in "${sorted_result[@]}"; do
	if ((v + 70000000 - total > 30000000)); then
		echo "$v"
		break
	fi
done"#
    )?;
    for (mut i, script) in [script1, script2].iter().enumerate() {
        i += 1;
        let solution_path = PathBuf::from(format!("generated_solution{i}.sh"));
        File::create(&solution_path)?.write_all(script.as_bytes())?;
        println!("Part{i} saved to: {}", solution_path.to_string_lossy());
    }
    Ok(())
}
