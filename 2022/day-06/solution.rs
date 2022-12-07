#!/usr/bin/env rust-script

use std::collections::HashSet;
use std::error::Error;
use std::fs;

const INPUT_FILE: &str = "input";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let find_marker = |message_size: usize, input: &str| -> Option<usize> {
        (message_size..input.len()).find(|&i| {
            input[i - message_size..i]
                .chars()
                .fold(HashSet::<char>::new(), |mut char_set, c| {
                    char_set.insert(c);
                    char_set
                })
                .len()
                == message_size
        })
    };

    println!(
        "Part1: {}",
        find_marker(4, &input).expect("failed to solve part1")
    );
    println!(
        "Part2: {}",
        find_marker(14, &input).expect("failed to solve part2")
    );

    Ok(())
}
