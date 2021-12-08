//! Day 3: Binary Diagnostic, Advent of Code 2021
//! https://adventofcode.com/2021/day/3
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::ops::{Deref, Not};
use std::str::FromStr;

use anyhow::{anyhow, bail};

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let input = parse_input(input_reader).expect("cannot parse input");

    // Part 1: power consumption computation
    let p1_answer = compute_power_consumption(input.as_slice())
        .expect("error while computing power consumption");
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: life support rating computation
    let p2_answer = compute_life_support_rating(input.as_slice())
        .expect("error while computing life support rating");
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses the sequence of bit vectors as a vector of specialized struct.
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<BitVec>> {
    reader.lines().map(|line| line?.parse()).collect()
}

/// Bit vector wrapper.
/// TODO: attempts to use [`bitvec::BitVec`] instead.
///
/// [`bitvec::BitVec`]: https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html
#[derive(Debug, Clone)]
struct BitVec(Vec<bool>);

// NOTE: I cannot figure out how to get `impl From<_> for T` to work
// for generic T: num::PrimInt + num::Unsigned. So macros for now.
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
impl_from_bitvec_for_int!(usize, u8, u16, u32, u64, u128);

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
/// -  gamma = radix majority voting of bit vector data
/// -  epsilon = radix minority voting of bit vector data
fn compute_power_consumption(numbers: &[BitVec]) -> anyhow::Result<u64> {
    let bit_length = numbers.iter().map(|v| v.len()).max();
    let bit_length = bit_length.ok_or(anyhow!("empty collection of bit vectors"))?;
    let gamma: BitVec = (0..bit_length)
        .map(|index| cast_votes(numbers.iter().collect::<Vec<_>>().as_slice(), index))
        .collect::<anyhow::Result<_>>()?;
    let epsilon: BitVec = gamma.iter().copied().map(|d| d.not()).collect();
    Ok(u64::from(&gamma) * u64::from(&epsilon))
}

/// Compute the life support rating, which is the product of these two factors:
/// -  oxygen generator rating = multi-round rotating majority vote
/// -  CO₂ scrubber rating = multi-round rotating minority vote
fn compute_life_support_rating(numbers: &[BitVec]) -> anyhow::Result<u64> {
    let oxygen_generator_rating = eliminate_until_last(numbers, cast_votes)?;
    let co2_scrubber_rating = eliminate_until_last(numbers, |numbers, index| {
        cast_votes(numbers, index).map(|d| d.not())
    })?;
    Ok(u64::from(oxygen_generator_rating) * u64::from(co2_scrubber_rating))
}

/// Performs multi-round elimination among all bit vectors until one survivor prevails.
/// Each round i, the remaining candidates compares i-th digit according to the tally criterion.
/// Candidates matching the result of the tally criterion survives to the next round.
///
/// Previous implementation of this function used [`itertools::Itertools::fold_while`],
/// which could have been an elegant solution.
/// However, with error handling plus short circuiting, the source code became too complex to understand,
/// and it is unclear if there is a performance gain to be expected.
///
/// [`itertools::Itertools::fold_while`]: https://docs.rs/itertools/0.10.3/itertools/trait.Itertools.html#method.fold_while
fn eliminate_until_last<F>(numbers: &[BitVec], vote_criterion: F) -> anyhow::Result<&BitVec>
where
    F: Fn(&[&BitVec], usize) -> anyhow::Result<bool>,
{
    let mut survivors: Vec<_> = numbers.iter().collect();
    for index in 0_usize.. {
        if survivors.len() <= 1 {
            break;
        }
        let vote_result = vote_criterion(survivors.as_slice(), index)?;
        survivors = survivors
            .into_iter()
            .filter(|num| num[index] == vote_result)
            .collect();
    }
    survivors
        .get(0)
        .copied()
        .ok_or(anyhow!("empty collection of bit vectors given"))
}

/// Fetches the votes from all bit vectors by indexing into each bit vector,
/// and determine the majority boolean result. Returns `true` in case of a tie.
/// TODO: change signature into an iterator producing &BitVec instead
///       (may need to look up on Higher-Rank Trait Bounds)
fn cast_votes(numbers: &[&BitVec], index: usize) -> anyhow::Result<bool> {
    let tally: isize = numbers
        .iter()
        .map(|num| {
            let vote = num.get(index).copied().map(|b| if b { 1 } else { -1 });
            vote.ok_or_else(|| anyhow!("index {} out of bounds for string {}", index, num))
        })
        .sum::<anyhow::Result<_>>()?;
    Ok(tally >= 0)
}
