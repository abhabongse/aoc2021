//! Day 6: Lanternfish, Advent of Code 2021
//! <https://adventofcode.com/2021/day/6>
use std::io;
use std::io::BufRead;

use anyhow::anyhow;
use nalgebra::{matrix, vector, SVector};

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let fish_attrs = parse_input(input_reader).expect("cannot parse input");

    let init_counts = count_fishes_by_attr(fish_attrs.as_slice()).expect("invalid fish attributes");
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
    // NOTE: I could have used repeated squaring exponentiation method
    // if the number of days happens to be much larger
    let p2_fish_counts = (0..256).fold(init_counts, |v, _| next_day_trans * v);
    let p2_total = p2_fish_counts.dot(&vector![1, 1, 1, 1, 1, 1, 1, 1, 1]);
    println!("Part 2 answer: {}", p2_total);
}

/// Parses the initial assignments of lanternfish in the sea.
/// TODO: Learn how to parse input from buffer stream with proper short-circuit error handling
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<usize>> {
    reader
        .lines()
        .collect::<Result<Vec<_>, io::Error>>()?
        .iter()
        .map(|line| line.split(','))
        .flatten()
        .map(|token| token.trim().quickparse())
        .collect()
}

/// Counts the number of fishes by their attributes
fn count_fishes_by_attr<const MAX_ATTR: usize>(
    fish_attrs: &[usize],
) -> anyhow::Result<SVector<u64, MAX_ATTR>> {
    let mut init_counts: SVector<u64, MAX_ATTR> = SVector::zeros();
    fish_attrs
        .iter()
        .copied()
        .try_for_each(|attr| -> anyhow::Result<()> {
            let attr_count = init_counts.get_mut(attr).ok_or_else(|| {
                anyhow!("fish attribute {} exceed limit of {}", attr, MAX_ATTR - 1)
            })?;
            *attr_count += 1;
            Ok(())
        })?;
    Ok(init_counts)
}
