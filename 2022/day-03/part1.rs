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
    let mut rucksacks = Vec::new();
    for line in input.lines() {
        let (c1, c2) = line.split_at(line.len() / 2);
        let c1 = parse_compartments(c1);
        let c2 = parse_compartments(c2);
        let intersection: u32 = (*c1
            .intersection(&c2)
            .into_iter()
            .next()
            .expect("cannot find intersection"))
        .into();
        rucksacks.push(intersection);
    }
    println!("Answer: {}", rucksacks.iter().sum::<u32>());
    Ok(())
}
