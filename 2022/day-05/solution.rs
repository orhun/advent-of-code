#!/usr/bin/env rust-script

use std::collections::LinkedList;
use std::error::Error;
use std::fs;

const INPUT_FILE: &str = "input";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();

    let mut crates = input_parts[0].lines().collect::<Vec<&str>>();
    let crate_count = crates.pop().unwrap().trim().split("   ").count();
    let mut stacks = vec![LinkedList::<char>::new(); crate_count];
    for current_crate in crates.iter() {
        for (i, stack) in stacks.iter_mut().enumerate().take(crate_count) {
            let c = current_crate.chars().nth(i + (i * 3) + 1).unwrap();
            if c != ' ' {
                stack.push_back(c);
            }
        }
    }

    let moves = input_parts[1]
        .lines()
        .map(|v| {
            v.replace(|c: char| !(c.is_numeric() || c == ' '), "")
                .split_whitespace()
                .flat_map(|v| v.parse())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut stacks1 = stacks.clone();
    for current_move in moves.iter() {
        for _ in 0..current_move[0] {
            let v = stacks1[current_move[1] - 1].pop_front().unwrap();
            stacks1[current_move[2] - 1].push_front(v);
        }
    }

    let mut stacks2 = stacks.clone();
    for current_move in moves {
        let mut ordered = LinkedList::new();
        for _ in 0..current_move[0] {
            ordered.push_back(stacks2[current_move[1] - 1].pop_front().unwrap());
        }
        ordered.append(&mut stacks2[current_move[2] - 1]);
        stacks2[current_move[2] - 1] = ordered;
    }

    for (i, stacks) in [stacks1, stacks2].iter().enumerate() {
        println!(
            "Part{i}: {}",
            stacks
                .iter()
                .flat_map(|v| v.iter().next())
                .collect::<String>()
        );
    }

    Ok(())
}
