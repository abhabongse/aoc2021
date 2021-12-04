//! Day 1: Sonar Sweep, Advent of Code 2021
//! https://adventofcode.com/2021/day/1
use std::io::BufRead;

use itertools::Itertools;

use aoc2021::argparser;

fn main() {
    let input_file = std::env::args().nth(1).unwrap_or_else(|| "-".into());
    let reader = argparser::reader_from_file(input_file).expect("cannot open file");
    let input = parse_input(reader).expect("cannot parse input");

    let p1_answer: usize = input
        .iter()
        .tuple_windows()
        .map(|(x, y)| (y > x) as usize)
        .sum();
    println!("Part 1 answer: {}", p1_answer);

    let p2_answer: usize = input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(x, y)| (y > x) as usize)
        .sum();
    println!("Part 2 answer: {}", p2_answer);
}

fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<isize>> {
    reader
        .lines()
        .map(|line| Ok(line?.trim().parse()?))
        .collect()
}
