#!/usr/bin/env rust-script

use std::error::Error;
use std::fs;

const INPUT_FILE: &str = "input";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let parse_assignments = |s: &str| -> Vec<u32> {
        let range = s
            .split('-')
            .map(|v| v.parse().ok())
            .flatten()
            .collect::<Vec<u32>>();
        (range[0]..=range[1]).collect()
    };
    let mut all_overlaps = 0;
    let mut any_overlaps = 0;
    for line in input.lines() {
        let assignments = line.split(',').collect::<Vec<&str>>();
        let assignment1 = parse_assignments(assignments[0]);
        let assignment2 = parse_assignments(assignments[1]);
        if assignment1.iter().all(|item| assignment2.contains(item))
            || assignment2.iter().all(|item| assignment1.contains(item))
        {
            all_overlaps += 1;
        }
        if assignment1.iter().any(|item| assignment2.contains(item))
            || assignment2.iter().any(|item| assignment1.contains(item))
        {
            any_overlaps += 1;
        }
    }
    println!("Part1: {}, Part2: {}", all_overlaps, any_overlaps);
    Ok(())
}
