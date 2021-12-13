//! Day 6: Lanternfish, Advent of Code 2021
//! <https://adventofcode.com/2021/day/6>
use std::io;
use std::io::BufRead;

use anyhow::anyhow;
// TODO: Stop using nalgebra, use homegrown grid
use nalgebra::{matrix, vector, SVector};

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let fish_attrs = parse_input(input_reader).expect("cannot parse input");

    let init_counts = count_fishes_by_attr(fish_attrs.as_slice()).expect("invalid fish attributes");
    let trans_mat = matrix![
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
    let p1_total_fish = {
        let fish_counts = (0..80).fold(init_counts, |v, _| trans_mat * v);
        fish_counts.dot(&vector![1, 1, 1, 1, 1, 1, 1, 1, 1])
    };
    println!("Part 1 answer: {}", p1_total_fish);

    // Part 2: fish counting after 256 days
    // NOTE: I could have used repeated squaring exponentiation method to reduce some time
    // if the number of days happened to be much larger than this.
    let p2_total_fish = {
        let fish_counts = (0..256).fold(init_counts, |v, _| trans_mat * v);
        fish_counts.dot(&vector![1, 1, 1, 1, 1, 1, 1, 1, 1])
    };
    println!("Part 2 answer: {}", p2_total_fish);
}

/// Parses the initial assignments of lanternfish in the sea.
/// - TODO: Learn how to parse input from buffer stream with proper short-circuit error handling
fn parse_input<BR: BufRead>(reader: BR) -> anyhow::Result<Vec<usize>> {
    let lines: Vec<_> = reader.lines().collect::<Result<_, io::Error>>()?;
    lines
        .iter()
        .flat_map(|line| line.split(','))
        .map(|token| token.trim().quickparse())
        .collect()
}

/// Counts the number of fishes by their attributes
///
/// # Implementation Note
/// I did not use [`Itertools::counts`] since I want to be able to detect out-of-bounds indexing.
///
/// [`Itertools::counts`]: https://docs.rs/itertools/0.10.3/itertools/trait.Itertools.html#method.counts
fn count_fishes_by_attr<const MAX_ATTR: usize>(
    fish_attrs: &[usize],
) -> anyhow::Result<SVector<u64, MAX_ATTR>> {
    let mut init_counts: SVector<u64, MAX_ATTR> = SVector::zeros();
    for attr in fish_attrs.iter().copied() {
        let count = init_counts
            .get_mut(attr)
            .ok_or_else(|| anyhow!("fish attribute {} exceed limit of {}", attr, MAX_ATTR - 1))?;
        *count += 1;
    }
    Ok(init_counts)
}
