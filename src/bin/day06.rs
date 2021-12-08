//! Day 6: Lanternfish, Advent of Code 2021
//! https://adventofcode.com/2021/day/6
use std::io;
use std::io::BufRead;

use nalgebra::{matrix, vector, SVector};

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let fish_attrs = parse_input(input_reader).expect("cannot parse input");

    let init_counts = {
        let mut init_counts: SVector<u64, 9> = SVector::zeros();
        fish_attrs.iter().copied().for_each(|a| init_counts[a] += 1);
        init_counts
    };
    let next_day_trans = matrix![
        0, 1, 0, 0, 0, 0, 0, 0, 0;
        0, 0, 1, 0, 0, 0, 0, 0, 0;
        0, 0, 0, 1, 0, 0, 0, 0, 0;
        0, 0, 0, 0, 1, 0, 0, 0, 0;
        0, 0, 0, 0, 0, 1, 0, 0, 0;
        0, 0, 0, 0, 0, 0, 1, 0, 0;
        1, 0, 0, 0, 0, 0, 0, 1, 0;
        0, 0, 0, 0, 0, 0, 0, 0, 1;
        1, 0, 0, 0, 0, 0, 0, 0, 0;
    ];

    // Part 1: fish counting after 80 days
    let p1_fish_counts = (0..80).fold(init_counts, |v, _| next_day_trans * v);
    let p1_total = p1_fish_counts.dot(&vector![1, 1, 1, 1, 1, 1, 1, 1, 1]);
    println!("Part 1 answer: {}", p1_total);

    // Part 1: fish counting after 256 days
    // NOTE: could have used repeated squaring exponentiation method
    // if the number of days happens to be much larger
    let p2_fish_counts = (0..256).fold(init_counts, |v, _| next_day_trans * v);
    let p2_total = p2_fish_counts.dot(&vector![1, 1, 1, 1, 1, 1, 1, 1, 1]);
    println!("Part 2 answer: {}", p2_total);
}

/// Parses the initial assignments of lanternfish in the sea.
/// TODO: learn how to parse input in a more stream-friendly manner
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<usize>> {
    reader
        .lines()
        .collect::<Result<Vec<_>, io::Error>>()?
        .iter()
        .map(|line| line.split(','))
        .flatten()
        .map(|token| Ok(token.trim().parse()?))
        .collect()
}