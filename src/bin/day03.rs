//! Day 3: Binary Diagnostic, Advent of Code 2021
//! https://adventofcode.com/2021/day/3
use std::io::BufRead;
use std::ops::{Deref, Not};
use std::str::FromStr;

use anyhow::bail;
use itertools::{FoldWhile, Itertools, MinMaxResult};

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let input = parse_input(input_reader).expect("cannot parse input");

    let p1_answer = compute_power_consumption(input.as_slice())
        .expect("error while computing power consumption");
    println!("Part 1 answer: {}", p1_answer);

    let p2_answer = compute_life_support_rating(input.as_slice())
        .expect("error while computing life support rating");
    println!("Part 2 answer: {}", p2_answer);
}

fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<BitVec>> {
    reader.lines().map(|line| line?.parse()).collect()
}

/// Bit vector wrapper
#[derive(Debug, Clone)]
struct BitVec(Vec<bool>);

impl FromStr for BitVec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: anyhow::Result<Vec<_>> = s
            .trim()
            .bytes()
            .map(|c| match c {
                b'0' => Ok(false),
                b'1' => Ok(true),
                _ => bail!("invalid character in bit string: {}", c),
            })
            .collect();
        Ok(BitVec(inner?))
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        BitVec(iter.into_iter().collect())
    }
}

impl Deref for BitVec {
    type Target = Vec<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Computes the power consumption, which is the product of these two factors:
/// -  gamma = radix majority voting of bit vector data
/// -  epsilon = radix minority voting of bit vector data
fn compute_power_consumption(data: &[BitVec]) -> anyhow::Result<usize> {
    let bit_length = common_bit_length(data)?;
    let gamma: BitVec = (0..bit_length)
        .map(|i| majority_vote(data.iter().map(|v| v[i])))
        .collect();
    let epsilon: BitVec = gamma.iter().map(|c| c.not()).collect();
    Ok(bitvec_to_integer(&gamma) * bitvec_to_integer(&epsilon))
}

/// Compute the life support rating, which is the product of these two factors:
/// -  oxygen generator rating = multi-round rotating majority vote
/// -  CO₂ scrubber rating = multi-round rotating minority vote
fn compute_life_support_rating(data: &[BitVec]) -> anyhow::Result<usize> {
    let oxygen_generator_rating =
        eliminate_until_last(data, |votes| majority_vote(votes.iter().copied()));
    let co2_scrubber_rating =
        eliminate_until_last(data, |votes| majority_vote(votes.iter().copied()).not());
    Ok(bitvec_to_integer(oxygen_generator_rating) * bitvec_to_integer(co2_scrubber_rating))
}

/// Performs multi-round elimination among all bit vectors until one survivor prevails.
/// Each round i, the remaining candidates compares i-th digit according to the tally criterion.
/// Candidates matching the result of the tally criterion survives to the next round.
/// TODO: fix the potentially panic scenario when boolean indexing happens out-of-bounds.
fn eliminate_until_last<F: Fn(&[bool]) -> bool>(data: &[BitVec], tally_criterion: F) -> &BitVec {
    let candidates: Vec<_> = (0..data.len()).collect();
    let last_survivor = (0usize..)
        .fold_while(candidates, |remaining, i| {
            if remaining.len() <= 1 {
                FoldWhile::Done(remaining)
            } else {
                let votes: Vec<_> = remaining.iter().map(|r| data[*r][i]).collect();
                let vote_result = tally_criterion(votes.as_slice());
                let survivors: Vec<_> = remaining
                    .into_iter()
                    .filter(|r| data[*r][i] == vote_result)
                    .collect();
                FoldWhile::Continue(survivors)
            }
        })
        .into_inner();
    &data[last_survivor[0]]
}

/// Tallies the votes and returns the majority boolean. Returns true is case of a tie.
fn majority_vote<I: Iterator<Item = bool>>(votes: I) -> bool {
    votes.map(|v| if v { 1 } else { -1 }).sum::<isize>() >= 0
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
fn bitvec_to_integer(s: &BitVec) -> usize {
    s.iter().fold(0, |acc, val| 2 * acc + (*val) as usize)
}
