//! Day 7: The Treachery of Whales, Advent of Code 2021
//! <https://adventofcode.com/2021/day/7>
use std::io;
use std::io::BufRead;

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let mut positions = parse_input(input_reader).expect("cannot parse input");

    // The rest of the code assumes that all positions are sorted.
    positions.sort_unstable();

    // Part 1: fuels from distance according to linear function (at right-heavy median point)
    let p1_fuels: i64 = {
        let median = positions[positions.len() / 2];
        positions
            .iter()
            .copied()
            .map(|pos| const_per_unit_dist_fuel(pos, median))
            .sum()
    };
    println!("Part 1 answer: {}", p1_fuels);

    // Part 2: fuels from distance according to triangle shape accumulation
    // NOTE: To be honest, I don't really know if checking only the neighboring values
    // of the mean position as the candidate positions are sufficient to find the optimal answer.
    // A few other potential alternative solutions (must validate assumptions first):
    // -  Using binary search, assuming that the fuel function is a unimodal function
    // -  Using golden-section search, assuming that the fuel function is a convex function
    let p2_fuels: i64 = {
        let mean = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
        [mean.floor() as i64, mean.ceil() as i64]
            .into_iter()
            .map(|mean| {
                positions
                    .iter()
                    .copied()
                    .map(|pos| linear_per_unit_dist_fuel(pos, mean))
                    .sum()
            })
            .min()
            .unwrap()
    };
    println!("Part 2 answer: {}", p2_fuels);
}

/// Parses the initial assignments of lanternfish in the sea.
/// TODO: Learn how to parse input from buffer stream with proper short-circuit error handling
fn parse_input<BR: BufRead>(reader: BR) -> anyhow::Result<Vec<i64>> {
    reader
        .lines()
        .collect::<Result<Vec<_>, io::Error>>()?
        .iter()
        .flat_map(|line| line.split(','))
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
