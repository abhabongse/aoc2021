//! Day 1: Sonar Sweep, Advent of Code 2021
//! https://adventofcode.com/2021/day/1
use std::io::BufRead;

use itertools::Itertools;

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let input = parse_input(input_reader).expect("cannot parse input");

    let p1_inc_count: usize = input
        .iter()
        .tuple_windows()
        .map(|(x, y)| (y > x) as usize)
        .sum();
    println!("Part 1 answer: {}", p1_inc_count);

    let p2_inc_count: usize = input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(x, y)| (y > x) as usize)
        .sum();
    println!("Part 2 answer: {}", p2_inc_count);
}

/// Parses the report (program input) as a vector of integers.
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<i64>> {
    reader
        .lines()
        .map(|line| Ok(line?.trim().parse()?))
        .collect()
}
