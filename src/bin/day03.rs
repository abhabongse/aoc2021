//! Day 3: Binary Diagnostic, Advent of Code 2021
//! https://adventofcode.com/2021/day/3
use std::cmp::Ordering;
use std::io::BufRead;

use anyhow::bail;
use itertools::{Itertools, MinMaxResult};

use aoc2021::argparser;

type BitVec = Vec<bool>;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.to_reader().expect("cannot open file");
    let input = parse_input(input_reader).expect("cannot parse input");

    let p1_answer = compute_power_consumption(input.as_slice())
        .expect("error while computing power consumption");
    println!("Part 1 answer: {}", p1_answer);

    let p2_answer = compute_life_support_rating(input.as_slice())
        .expect("error while computing life support rating");
    println!("Part 2 answer: {}", p2_answer);
}

fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<BitVec>> {
    reader
        .lines()
        .map(|line| {
            line?
                .trim()
                .bytes()
                .map(|c| match c {
                    b'0' => Ok(false),
                    b'1' => Ok(true),
                    _ => bail!("invalid character in bitstring: {}", c),
                })
                .collect::<anyhow::Result<BitVec>>()
        })
        .collect()
}

/// Computes the power consumption, which is the product of these two factors:
/// - gamma = radix majority voting of bit vector data
/// - epsilon = radix minority voting of bit vector data
fn compute_power_consumption(data: &[BitVec]) -> anyhow::Result<usize> {
    let bit_length = common_bit_length(data)?;
    let zipped_result = (0..bit_length)
        .map(|i| {
            let tug_result: isize = data.iter().map(|v| if v[i] { 1 } else { -1 }).sum();
            match tug_result.cmp(&0) {
                Ordering::Greater => Ok((true, false)),
                Ordering::Less => Ok((false, true)),
                Ordering::Equal => bail!("cannot decide strict majority or minority"),
            }
        })
        .collect::<anyhow::Result<Vec<_>>>();

    let (gamma, epsilon): (BitVec, BitVec) = zipped_result?.iter().copied().unzip();
    let gamma = bitvec_to_integer(&gamma);
    let epsilon = bitvec_to_integer(&epsilon);
    Ok(gamma * epsilon)
}

/// Computes the common bit length among the collection of bit vectors.
/// An error occurs if they disagree.
fn common_bit_length(data: &[BitVec]) -> anyhow::Result<usize> {
    let minmax_result = data.iter().map(|v| v.len()).minmax();
    match minmax_result {
        MinMaxResult::NoElements => bail!("empty collection of bit vectors"),
        MinMaxResult::MinMax(min, max) if min != max => {
            bail!("non-uniform length bit vectors between {} and {}", min, max)
        }
        MinMaxResult::MinMax(l, _) | MinMaxResult::OneElement(l) => Ok(l),
    }
}

/// Calculates the integer representation of the given bit vector in MSB-first order.
#[allow(clippy::ptr_arg)]
fn bitvec_to_integer(s: &BitVec) -> usize {
    s.iter().fold(0, |acc, val| 2 * acc + (*val) as usize)
}

/// Compute the life support rating, which is the product of these two factors:
/// - oxygen generator rating = multi-round rotating majority vote
/// - CO₂ scrubber rating = multi-round rotating minority vote
fn compute_life_support_rating(data: &[BitVec]) -> anyhow::Result<usize> {
    todo!()
}
