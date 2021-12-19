//! Day 6: Lanternfish, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/6>
use std::io::BufRead;

use anyhow::Context;
use nalgebra::{matrix, SVector};

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { fish_attrs } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Initialize fish counts by their attributes
    let init_counts = count_fishes_by_attr(fish_attrs.as_slice()).expect("invalid fish attributes");

    // Transformation matrix representing how fish reproduces
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
        fish_counts.sum()
    };
    println!("Part 1 answer: {}", p1_total_fish);

    // Part 2: fish counting after 256 days
    // NOTE: I could have used repeated squaring exponentiation method to reduce some time
    // if the number of days happened to be much larger than this.
    let p2_total_fish = {
        let fish_counts = (0..256).fold(init_counts, |v, _| trans_mat * v);
        fish_counts.sum()
    };
    println!("Part 2 answer: {}", p2_total_fish);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Initial attributes of lanternfish in the sea
    fish_attrs: Vec<usize>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut fish_attrs = Vec::new();
        for line in reader.lines() {
            for token in line?.split(',') {
                fish_attrs.push(token.trim().quickparse()?);
            }
        }
        Ok(Input { fish_attrs })
    }
}

/// Counts the number of fishes by their attributes.
///
/// # Implementation Note
/// I did not use [`Itertools::counts`] since I want to be able to detect out-of-bounds indexing.
///
/// [`Itertools::counts`]: https://docs.rs/itertools/0.10.3/itertools/trait.Itertools.html#method.counts
fn count_fishes_by_attr<const M: usize>(fish_attrs: &[usize]) -> anyhow::Result<SVector<u64, M>> {
    let mut counts: SVector<u64, M> = SVector::zeros();
    for attr in fish_attrs.iter().copied() {
        let count_mut = counts
            .get_mut(attr)
            .with_context(|| format!("fish attribute {} exceed limit of {}", attr, M - 1))?;
        *count_mut += 1;
    }
    Ok(counts)
}
