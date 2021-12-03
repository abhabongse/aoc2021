//! Day 1: Sonar Sweep, Advent of Code 2021
//! https://adventofcode.com/2021/day/1
use std::io::BufRead;
use std::path::PathBuf;

use itertools::Itertools;
use structopt::StructOpt;

use aoc2021::argparser;

/// Program input options for Advent of Code solution files.
/// It optionally expects a single positional argument locating the input file!
#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to input file. Omit or specify '-' to use standard input.
    #[structopt(name = "INPUT_FILE", default_value = "-", parse(from_os_str))]
    input_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let reader = argparser::reader_from_file(opt.input_file).expect("cannot open file");
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
