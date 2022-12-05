#!/usr/bin/env rust-script

use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;

const INPUT_FILE: &str = "input";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let parse_compartments = |c: &str| -> HashSet<u8> {
        HashSet::<u8>::from_iter(
            c.chars()
                .map(|v| v as u8 - if v.is_lowercase() { 96 } else { 38 })
                .collect::<Vec<u8>>(),
        )
    };
    let mut priorities = Vec::new();
    for groups in input.lines().collect::<Vec<&str>>().chunks(3) {
        let groups: Vec<HashSet<u8>> = groups.iter().map(|v| parse_compartments(v)).collect();
        priorities.push(
            groups
                .iter()
                .fold(HashSet::<u8>::new(), |mut groups, group| {
                    if !groups.is_empty() {
                        groups = groups.intersection(group).copied().collect();
                    } else {
                        groups = group.clone();
                    }
                    groups
                })
                .into_iter()
                .next()
                .expect("cannot find intersection")
                .into(),
        );
    }
    println!("Answer: {}", priorities.iter().sum::<u32>());
    Ok(())
}
