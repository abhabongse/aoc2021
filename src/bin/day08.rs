//! Day 8: Seven Segment Search, Advent of Code 2021
//! <https://adventofcode.com/2021/day/8>
use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser;
use aoc2021::collections::TryCollectArray;

/// Hand-crafted information to decode toggle patterns into actual integer digits.
/// In general, it performs an XOR-bitmask-then-count-one-bits test with each pattern.
///
/// Specifically, each `i`-th triplet of this static variable array `(null, one, four)`
/// precisely decodes a `pattern` into integer digit `i`, if and only if:
/// -  `pattern` contains exactly `null` one-bits
/// -  `pattern ^ pattern_one` contains exactly `one` one-bits
///    where `pattern_one` is the toggle pattern which decodes to digit 1
/// -  `pattern ^ pattern_four` contains exactly `four` one-bits
///    where `pattern_four` is the toggle pattern which decodes to digit 4
static DECODER_BY_NULL_ONE_FOUR: [(u32, u32, u32); 10] = [
    (6, 4, 4),
    (2, 0, 2),
    (5, 5, 5),
    (5, 3, 3),
    (4, 2, 0),
    (5, 5, 3),
    (6, 6, 4),
    (3, 1, 3),
    (7, 5, 3),
    (6, 4, 2),
];

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let display_logs = parse_input(input_reader).expect("cannot parse input");

    // Part 1: Counting appearances of displaying digits with unique number of segments.
    let p1_answer: usize = display_logs
        .iter()
        .map(|log| log.count_quickly_decodable_display_patterns())
        .sum();
    println!("Part 1 answer: {}", p1_answer);
    // Part 2: Decode four-digit displaying numbers and add them up
    let p2_answer: u64 = display_logs
        .iter()
        .map(|log| log.decode_display_patterns())
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("error occurred while decoding display patterns")
        .into_iter()
        .sum();
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses the initial assignments of lanternfish in the sea.
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<DisplayLog>> {
    reader
        .lines()
        .map(|line| line.context("cannot read a line of string")?.parse())
        .collect()
}

/// `DisplayLog` consists of 10 signal patterns and 4 digit output patterns of a display.
/// Each toggle pattern of a seven-segment digit display is represented
/// by an 8-bit unsigned integer (but only 7 of them are used).
#[derive(Debug, Clone)]
struct DisplayLog {
    digit_patterns: [u8; 10],
    display_patterns: [u8; 4],
}

impl DisplayLog {
    /// Constructs a new [`DisplayLog`] but with `signal_patterns` properly sorted.
    fn new(digit_patterns: [u8; 10], display_patterns: [u8; 4]) -> Self {
        DisplayLog {
            digit_patterns: sort_toggle_patterns(&digit_patterns),
            display_patterns,
        }
    }

    /// Decodes the toggle `pattern` into an integer digit.
    fn decode_pattern(&self, pattern: u8) -> Option<u64> {
        self.digit_patterns
            .iter()
            .copied()
            .find_position(|p| *p == pattern)
            .map(|(pos, _)| pos as u64)
    }

    /// Does the toggle `pattern` contains the number of one-bits
    /// (i.e. the number of lit up segments in the seven-segment digit display)
    /// which immediately uniquely identifies an integer digit (1, 4, 7, or 8).
    /// An error returned by this function indicates an unrecognizable toggle pattern.
    fn can_quickly_decode(&self, pattern: u8) -> anyhow::Result<bool> {
        let digit = self
            .decode_pattern(pattern)
            .ok_or_else(|| anyhow!("cannot decode toggle pattern: {:02b}", pattern))?;
        Ok([1, 4, 7, 8].into_iter().any(|target| digit == target))
    }

    /// Counts the number of toggle patterns within the output displays
    /// that can unique identify a digit solely on the number of one-bits
    /// (i.e. the number of lit up segments in the seven-segment digit display).
    fn count_quickly_decodable_display_patterns(&self) -> usize {
        self.display_patterns
            .iter()
            .copied()
            .filter(|p| self.can_quickly_decode(*p).unwrap_or(false))
            .count()
    }

    /// Decodes all digits of the display patterns into a four-digit number.
    fn decode_display_patterns(&self) -> anyhow::Result<u64> {
        self.display_patterns
            .iter()
            .copied()
            .try_fold(0, |acc, pattern| {
                Some(10 * acc + self.decode_pattern(pattern)?)
            })
            .ok_or(anyhow!("cannot decode output display patterns"))
    }
}

impl FromStr for DisplayLog {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)\s*
                    ([a-g]+)\s+([a-g]+)\s+([a-g]+)\s+([a-g]+)\s+([a-g]+)\s+
                    ([a-g]+)\s+([a-g]+)\s+([a-g]+)\s+([a-g]+)\s+([a-g]+)\s+
                    \|\s+
                    ([a-g]+)\s+([a-g]+)\s+([a-g]+)\s+([a-g]+)\s*"
            )
            .unwrap();
        }
        let captures = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid line display input: {}", s))?;
        let all_patterns: Vec<_> = (1..=14)
            .map(|i| pattern_from_scribbles(&captures[i]))
            .collect::<anyhow::Result<_>>()?;
        let digit_patterns = (&all_patterns[0..10])
            .iter()
            .copied()
            .try_collect_exact_array()?;
        let display_patterns = (&all_patterns[10..14])
            .iter()
            .copied()
            .try_collect_exact_array()?;

        Ok(DisplayLog::new(digit_patterns, display_patterns))
    }
}

/// Converts the log scribbles (a string combination of `'a'` through `'g'`) into a toggle pattern,
/// which is an 8-bit integer representation compatible with [`DisplayLog`].
fn pattern_from_scribbles<T: AsRef<str>>(scribbles: T) -> anyhow::Result<u8> {
    let mut pattern = 0;
    for c in scribbles.as_ref().bytes() {
        let pos = match c {
            b'a'..=b'g' => c - b'a',
            _ => bail!("invalid character: {}", c),
        };
        let mask = 1 << pos;
        if (pattern & mask) != 0 {
            bail!("duplicated character: {}", c);
        }
        pattern |= mask;
    }
    Ok(pattern)
}

/// Sorts the toggle patterns so that the `i`-th pattern precisely decodes to digit `i`.
fn sort_toggle_patterns(patterns: &[u8; 10]) -> [u8; 10] {
    let one_decoder = DECODER_BY_NULL_ONE_FOUR[1].0;
    let one_mask = pattern_by_xor_mask_tests(&patterns, [(0, one_decoder)].as_slice());
    let four_decoder = DECODER_BY_NULL_ONE_FOUR[4].0;
    let four_mask = pattern_by_xor_mask_tests(&patterns, [(0, four_decoder)].as_slice());
    DECODER_BY_NULL_ONE_FOUR.map(|(null, one, four)| {
        let tests = [(0, null), (one_mask, one), (four_mask, four)];
        pattern_by_xor_mask_tests(&patterns, tests.as_slice())
    })
}

/// Finds the first (and hopefully the only) toggle pattern
/// that satisfies all of XOR-bitmask-then-count-one-bits tests provided.
///
/// Each test consists of `(bit_mask, one_bits)`:
/// -  `bit_mask`: XOR bit mask which must be applied to a toggle pattern in question first
/// -  `one_bits`: expected number of one bits after masking the toggle pattern
fn pattern_by_xor_mask_tests(patterns: &[u8; 10], tests: &[(u8, u32)]) -> u8 {
    patterns
        .iter()
        .copied()
        .filter(|n| {
            tests
                .iter()
                .all(|test| (*n ^ test.0).count_ones() == test.1)
        })
        .exactly_one()
        .expect("expected exactly one element here")
}
