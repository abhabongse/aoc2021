//! Day 7: The Treachery of Whales, Advent of Code 2021
//! <https://adventofcode.com/2021/day/7>
use std::io;
use std::io::BufRead;

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let mut positions = parse_input(input_reader).expect("cannot parse input");

    // Part 1: fuels from distance according to linear function (at right-heavy median point)
    positions.sort_unstable();
    let p1_median = positions[positions.len() / 2];
    let p1_fuels: i64 = positions
        .iter()
        .copied()
        .map(|pos| const_per_unit_dist_fuel(pos, p1_median))
        .sum();
    println!("Part 1 answer: {}", p1_fuels);

    // Part 2: fuels from distance according to triangle shape accumulation
    // NOTE: To be honest, I don't really know if checking only the neighboring values
    // of the mean position as the candidate positions are sufficient to find the optimal answer.
    // A few other potential alternative solutions (must validate assumptions first):
    // -  Using binary search, assuming that the fuel function is a unimodal function
    // -  Using golden-section search, assuming that the fuel function is a convex function
    let p2_mean = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
    let p2_fuels: i64 = [p2_mean.floor() as i64, p2_mean.ceil() as i64]
        .iter()
        .copied()
        .map(|mean| {
            positions
                .iter()
                .copied()
                .map(|pos| linear_per_unit_dist_fuel(pos, mean))
                .sum()
        })
        .min()
        .unwrap();
    println!("Part 2 answer: {}", p2_fuels);
}

/// Parses the initial assignments of lanternfish in the sea.
/// TODO: Learn how to parse input from buffer stream with proper short-circuit error handling
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<i64>> {
    reader
        .lines()
        .collect::<Result<Vec<_>, io::Error>>()?
        .iter()
        .map(|line| line.split(','))
        .flatten()
        .map(|token| token.trim().quickparse())
        .collect()
}

/// Fuels required when using one fuel per distance unit.
fn const_per_unit_dist_fuel(p: i64, q: i64) -> i64 {
    (p - q).abs()
}

/// Fuels required when using linearly increasing amount of fuel for each unit of distance traveled.
/// This is essentially a triangle number based on the distances apart.
fn linear_per_unit_dist_fuel(p: i64, q: i64) -> i64 {
    let dist = (p - q).abs();
    dist * (dist + 1) / 2
}
