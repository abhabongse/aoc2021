//! Day 3: Binary Diagnostic, Advent of Code 2021
//! https://adventofcode.com/2021/day/3
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::ops::{Deref, Not};
use std::str::FromStr;

use anyhow::{anyhow, bail};
use itertools::{FoldWhile, Itertools};

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

impl BitVec {
    /// Calculates the integer representation of the given bit vector in MSB-first order.
    fn to_integer(&self) -> usize {
        usize::from_str_radix(self.to_string().as_str(), 2).unwrap()
    }
}

impl Display for BitVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().map(|v| if *v { '1' } else { '0' }).collect();
        write!(f, "{}", s)
    }
}

impl FromStr for BitVec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Vec<_> = s
            .trim()
            .bytes()
            .map(|c| match c {
                b'0' => Ok(false),
                b'1' => Ok(true),
                _ => bail!("invalid character in bit string: {}", c),
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(BitVec(inner))
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
fn compute_power_consumption(numbers: &[BitVec]) -> anyhow::Result<usize> {
    let bit_length = numbers.iter().map(|n| n.len()).max();
    let bit_length = bit_length.ok_or(anyhow!("empty collection of bit vectors"))?;
    let gamma: BitVec = (0..bit_length)
        .map(|index| cast_votes(numbers.iter().collect::<Vec<_>>().as_slice(), index))
        .collect::<anyhow::Result<_>>()?;
    let epsilon: BitVec = gamma.iter().map(|d| d.not()).collect();
    Ok(gamma.to_integer() * epsilon.to_integer())
}

/// Compute the life support rating, which is the product of these two factors:
/// -  oxygen generator rating = multi-round rotating majority vote
/// -  CO₂ scrubber rating = multi-round rotating minority vote
fn compute_life_support_rating(numbers: &[BitVec]) -> anyhow::Result<usize> {
    let oxygen_generator_rating = eliminate_until_last(numbers, cast_votes)?;
    let co2_scrubber_rating = eliminate_until_last(numbers, |numbers, index| {
        cast_votes(numbers, index).map(|d| d.not())
    })?;
    Ok(oxygen_generator_rating.to_integer() * co2_scrubber_rating.to_integer())
}

/// Performs multi-round elimination among all bit vectors until one survivor prevails.
/// Each round i, the remaining candidates compares i-th digit according to the tally criterion.
/// Candidates matching the result of the tally criterion survives to the next round.
fn eliminate_until_last<F>(numbers: &[BitVec], vote_criterion: F) -> anyhow::Result<&BitVec>
where
    F: Fn(&[&BitVec], usize) -> anyhow::Result<bool>,
{
    (0usize..)
        .fold_while(
            Ok(numbers.iter().collect::<Vec<_>>()), // initial participants
            |remaining, index| match remaining {
                Err(_) => FoldWhile::Done(remaining),
                Ok(candidates) if candidates.len() <= 1 => FoldWhile::Done(Ok(candidates)),
                Ok(candidates) => {
                    // One round of elimination
                    let vote_result = match vote_criterion(candidates.as_slice(), index) {
                        Ok(vote_result) => vote_result,
                        Err(err) => return FoldWhile::Done(Err(err)),
                    };
                    FoldWhile::Continue(Ok(candidates
                        .into_iter()
                        .filter(|n| n[index] == vote_result)
                        .collect()))
                }
            },
        )
        .into_inner()?
        .get(0)
        .copied()
        .ok_or(anyhow!("empty collection of bit vectors"))
}

/// Fetches the votes from all bit vectors by indexing into each bit vector,
/// and determine the majority boolean result. Returns `true` in case of a tie.
/// TODO: change signature into an iterator producing &BitVec instead
fn cast_votes(numbers: &[&BitVec], index: usize) -> anyhow::Result<bool> {
    let tally: isize = numbers
        .iter()
        .map(|n| {
            let score = n.get(index).map(|v| if *v { 1 } else { -1 });
            score.ok_or_else(|| anyhow!("index {} out of bounds for string {}", index, n))
        })
        .sum::<anyhow::Result<_>>()?;
    Ok(tally >= 0)
}
