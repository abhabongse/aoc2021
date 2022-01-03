//! Day 3: Binary Diagnostic, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/3>
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader};
use std::ops::{Deref, Not};
use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use itertools::Itertools;

use aoc2021::argparser::Cli;
use aoc2021::parsing::QuickParse;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { bit_vectors } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Obtain a vector to references to bit vectors
    let bit_vector_refs: Vec<_> = bit_vectors.iter().collect();

    // Part 1: Power consumption computation
    let p1_answer = compute_power_consumption(bit_vector_refs.as_slice())
        .expect("error while computing power consumption");
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Life support rating computation
    let p2_answer = compute_life_support_rating(bit_vector_refs.as_slice())
        .expect("error while computing life support rating");
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// List of bit vectors
    bit_vectors: Vec<BitVec>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut bit_vectors = Vec::new();
        for line in reader.lines() {
            bit_vectors.push(line?.quickparse()?);
        }
        Ok(Input { bit_vectors })
    }
}

/// Bit vector wrapper over a vector of boolean
///
/// # Implementation Note
/// This approach wastes significant amount of memory,
/// due to 8 bit being used to store a single boolean.
/// - TODO: Use [`bitvec::BitVec`] from external crate instead
///
/// [`bitvec::BitVec`]: https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html
#[derive(Debug, Clone)]
struct BitVec(Vec<bool>);

// NOTE: I cannot figure out how to get `impl From<_> for T` to work
// for generic T: num::PrimInt + num::Unsigned. So using macros for now.
macro_rules! impl_from_bitvec_for_int {
    ($($t:ty),*) => {$(
        impl From<&BitVec> for $t {
            fn from(num: &BitVec) -> Self {
                <$t>::from_str_radix(num.to_string().as_str(), 2).unwrap()
            }
        }
    )*};
    ($($t:ty,)*) => {
        impl_from_bitvec_for_int!( $($t:ty),* )
    };
}
impl_from_bitvec_for_int![usize, u8, u16, u32, u64, u128];

impl Display for BitVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .0
            .iter()
            .copied()
            .map(|b| if b { '1' } else { '0' })
            .collect();
        write!(f, "{}", s)
    }
}

impl FromStr for BitVec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = Vec::new();
        for c in s.trim().chars() {
            let d = c.to_digit(2).with_context(|| {
                format!("invalid character in bit string: '{}'", c.escape_default())
            })?;
            inner.push(d != 0);
        }
        Ok(BitVec(inner))
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
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
/// -  `gamma` = radix majority voting of bit vector data
/// -  `epsilon` = radix minority voting of the bit vector data
fn compute_power_consumption(numbers: &[&BitVec]) -> anyhow::Result<u64> {
    let bit_length = numbers.iter().map(|v| v.len()).max();
    let bit_length = bit_length.context("empty collection of bit vectors")?;
    let gamma: BitVec = (0..bit_length)
        .map(|index| cast_votes(numbers, index))
        .try_collect()?;
    let epsilon: BitVec = gamma.iter().copied().map(bool::not).collect();
    Ok(u64::from(&gamma) * u64::from(&epsilon))
}

/// Compute the life support rating, which is the product of these two factors:
/// -  **Oxygen Generator Rating** = multi-round, radix-rotating majority vote
/// -  **CO₂ scrubber rating** = multi-round, radix-rotating minority vote
fn compute_life_support_rating(numbers: &[&BitVec]) -> anyhow::Result<u64> {
    let o2_generator_rating = eliminate_until_last(numbers, cast_votes)?;
    let co2_scrubber_rating = eliminate_until_last(numbers, |numbers, index| {
        cast_votes(numbers, index).map(bool::not)
    })?;
    Ok(u64::from(o2_generator_rating) * u64::from(co2_scrubber_rating))
}

/// Performs multi-round elimination by voting, among all bit vectors until one survivor prevails.
/// For each round `i` starting from 0, the remaining candidates compares the `i`-th digit
/// and run the vote against the `vote_criterion` function.
/// Candidates whose `i`-th digit match the result of `vote_criterion` survive to the next round.
fn eliminate_until_last<'a, F>(
    numbers: &[&'a BitVec],
    vote_criterion: F,
) -> anyhow::Result<&'a BitVec>
where
    F: Fn(&[&BitVec], usize) -> anyhow::Result<bool>,
{
    let mut survivors: Vec<_> = numbers.iter().copied().collect();
    for index in 0_usize.. {
        if survivors.len() <= 1 {
            return survivors
                .get(0)
                .copied()
                .context("empty collection of bit vectors");
        }
        let vote_result = vote_criterion(survivors.as_slice(), index)?;
        survivors = survivors
            .into_iter()
            .filter(|num| num[index] == vote_result)
            .collect();
    }
    unreachable!()
}

/// Fetches the votes from all bit vectors by indexing into each bit vector,
/// and determine the majority boolean result. Returns `true` in case of a tie.
fn cast_votes(numbers: &[&BitVec], index: usize) -> anyhow::Result<bool> {
    let mut tally: isize = 0;
    for num in numbers.iter() {
        let vote = num
            .get(index)
            .with_context(|| format!("index {} out of bounds for string {}", index, num))?;
        tally += if *vote { 1 } else { -1 };
    }
    Ok(tally >= 0)
}
