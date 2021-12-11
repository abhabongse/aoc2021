//! Day 10: Syntax Scoring, Advent of Code 2021
//! <https://adventofcode.com/2021/day/10>
use std::io::BufRead;

use anyhow::Context;

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let lines = parse_input(input_reader).expect("cannot parse input");

    println!("{:?}", lines);

    // Part 1: TODO
    let p1_answer = 0;
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses the report (program input) as a vector of integers.
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<String>> {
    reader
        .lines()
        .map(|line| {
            Ok(line
                .context("cannot read a line of string")?
                .trim()
                .to_string())
        })
        .collect()
}
