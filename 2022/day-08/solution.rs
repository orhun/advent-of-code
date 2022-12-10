#!/usr/bin/env rust-script

use std::error::Error;
use std::fs;

const INPUT_FILE: &str = "input";

#[derive(Debug)]
struct Tree {
    height: u8,
    x: u32,
    y: u32,
}

impl Tree {
    fn is_on_edge(&self, x_max: u32, y_max: u32) -> bool {
        self.x == 0 || self.y == 0 || self.x == x_max - 1 || self.y == y_max - 1
    }

    fn process_location(&self, grid: &[Self]) -> Vec<(bool, u32)> {
        [
            grid.iter()
                .filter(|t| t.x == self.x && t.y < self.y)
                .rev()
                .collect::<Vec<&Tree>>(),
            grid.iter()
                .filter(|t| t.x == self.x && t.y > self.y)
                .collect::<Vec<&Tree>>(),
            grid.iter()
                .filter(|t| t.y == self.y && t.x < self.x)
                .rev()
                .collect::<Vec<&Tree>>(),
            grid.iter()
                .filter(|t| t.y == self.y && t.x > self.x)
                .collect::<Vec<&Tree>>(),
        ]
        .iter()
        .map(|grid| {
            let mut scenic_score = 0;
            for tree in grid {
                scenic_score += 1;
                if self.height <= tree.height {
                    return (false, scenic_score);
                }
            }
            (true, scenic_score)
        })
        .collect()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut trees = Vec::<Tree>::new();
    for (y, x_val) in input.lines().enumerate() {
        for (x, height) in x_val.chars().enumerate() {
            trees.push(Tree {
                height: height.to_string().parse()?,
                x: x as u32,
                y: y as u32,
            });
        }
    }

    let (x_max, y_max) = (
        input.lines().next().unwrap().to_string().len() as u32,
        input.lines().count() as u32,
    );

    let mut visible_trees = (x_max * 2) + (y_max * 2) - 4;
    let mut scenic_scores = Vec::<u32>::new();
    for tree in trees.iter().filter(|t| !t.is_on_edge(x_max, y_max)) {
        let location = tree.process_location(&trees);
        visible_trees += (location
            .iter()
            .filter(|(is_visible, _)| *is_visible)
            .count()
            != 0) as u32;
        scenic_scores.push(location.iter().fold(1, |mut score, (_, v)| {
            score *= v;
            score
        }));
    }

    println!("Part1: {visible_trees}");
    println!(
        "Part2: {}",
        scenic_scores.iter().max().expect("cannot solve part2")
    );

    Ok(())
}
