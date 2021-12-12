//! Day 11: Dumbo Octopus, Advent of Code 2021
//! <https://adventofcode.com/2021/day/11>
use std::collections::{HashSet, VecDeque};
use std::io;
use std::io::{BufRead, Write};
use std::ops::ControlFlow;

use anyhow::{anyhow, ensure, Context};

use aoc2021::argparser;
use aoc2021::grid::{king_step_neighbors, FixedGrid};

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let input = parse_input(input_reader).expect("cannot parse input");

    // Part 1: TODO
    let p1_answer: usize = 0;
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer: usize = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses the energy level of octopuses in 10 by 10 grid.
fn parse_input<BR: BufRead>(reader: BR) -> anyhow::Result<i64> {
    Ok(0)
}
