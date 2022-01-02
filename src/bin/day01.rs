//! Day 1: Sonar Sweep, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/1>
use std::io::{BufRead, BufReader};

use clap::Parser;
use itertools::Itertools;

use aoc2021::argparser::Cli;
use aoc2021::parsing::QuickParse;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { depths } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: One-point window depth increment counting
    let p1_inc_count: usize = depths
        .iter()
        .tuple_windows()
        .map(|(x, y)| (y > x) as usize)
        .sum();
    println!("Part 1 answer: {}", p1_inc_count);

    // Part 2: Three-point window depth increment counting
    let p2_inc_count: usize = depths
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(x, y)| (y > x) as usize)
        .sum();
    println!("Part 2 answer: {}", p2_inc_count);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// List of sonar sweep reports
    depths: Vec<i64>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut depths = Vec::new();
        for line in reader.lines() {
            depths.push(line?.trim().quickparse()?);
        }
        Ok(Input { depths })
    }
}
