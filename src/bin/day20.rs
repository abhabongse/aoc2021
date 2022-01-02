//! Day 20: Trench Map, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/20>
use std::io::{BufRead, BufReader};

use clap::Parser;

use aoc2021::argparser::Cli;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input {} = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: TODO
    let p1_answer = 0;
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        Ok(Input {})
    }
}
