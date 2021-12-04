//! Day 3: Binary Diagnostic, Advent of Code 2021
//! https://adventofcode.com/2021/day/3
use std::cmp::Ordering;
use std::io::BufRead;

use anyhow::bail;
use itertools::{Itertools, MinMaxResult};

use aoc2021::argparser;

fn main() {
    let input_file = std::env::args().nth(1).unwrap_or_else(|| "-".into());
    let reader = argparser::reader_from_file(input_file).expect("cannot open file");
    let input = parse_input(reader).expect("cannot parse input");

    let p1_answer = compute_power_consumption(input.as_ref()).unwrap();
    println!("Part 1 answer: {}", p1_answer);
}

fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<String>> {
    let data = reader
        .lines()
        .map(|line| Ok(line?.trim().to_string()))
        .collect::<anyhow::Result<Vec<_>>>();
    data
}

/// Compute the power consumption, which is the product of these two factors:
/// - gamma = radix majority voting of bit vector data
/// - epsilon = radix minority voting of bit vector data
fn compute_power_consumption(data: &[String]) -> anyhow::Result<usize> {
    let bit_length_minmax = data.iter().map(|v| v.as_bytes().len()).minmax();
    let bit_length = match bit_length_minmax {
        MinMaxResult::NoElements => bail!("cannot compute gamma/epsilon of empty data"),
        MinMaxResult::MinMax(p, q) if p != q => {
            bail!("data are of non-uniform length (between {} and {})", p, q)
        }
        MinMaxResult::MinMax(p, _) | MinMaxResult::OneElement(p) => p,
    };
    let zipped_result: anyhow::Result<Vec<_>> = (0..bit_length)
        .map(|i| {
            let tug_result: anyhow::Result<isize> = data
                .iter()
                .map(|v| match v.as_bytes()[i] {
                    b'0' => Ok(-1),
                    b'1' => Ok(1),
                    c => bail!("unrecognized character: {}", c as char),
                })
                .sum();
            match tug_result?.cmp(&0) {
                Ordering::Greater => Ok(('1', '0')),
                Ordering::Less => Ok(('0', '1')),
                Ordering::Equal => bail!("cannot decide strict majority/minority"),
            }
        })
        .collect();
    // TODO: lazily unzip iterator of 2-tuples into 2-tuple of vectors
    let (gamma, epsilon): (String, String) = zipped_result?.iter().copied().unzip();
    let gamma = usize::from_str_radix(gamma.as_str(), 2)?;
    let epsilon = usize::from_str_radix(epsilon.as_str(), 2)?;
    Ok(gamma * epsilon)
}

/// Compute the life support rating, which is the product of these two factors:
/// - oxygen generator rating = multi-round rotating majority vote
/// - CO₂ scrubber rating = multi-round rotating minority vote
fn compute_life_support_rating(data: &[String]) -> anyhow::Result<usize> {
    todo!()
}
