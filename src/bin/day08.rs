//! Day 8: Seven Segment Search, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/8>
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{bail, ensure, Context};
use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::Cli;
use aoc2021::collect_array::CollectArray;
use aoc2021::parsing::QuickParse;

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

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { display_logs } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Counting appearances of displaying digits with unique number of segments
    let p1_answer: usize = display_logs
        .iter()
        .map(DisplayLog::count_quickly_decodable_display_patterns)
        .sum();
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Decoding four-digit displaying numbers and add them up
    let p2_answer: u64 = {
        let numbers: Vec<_> = display_logs
            .iter()
            .map(DisplayLog::decode_display_patterns)
            .collect::<anyhow::Result<_>>()
            .expect("error occurred while decoding display patterns");
        numbers.into_iter().sum()
    };
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Seven segment display logs
    display_logs: Vec<DisplayLog>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut display_logs = Vec::new();
        for line in reader.lines() {
            display_logs.push(line?.quickparse()?);
        }
        Ok(Input { display_logs })
    }
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
            .find_position(|&p| p == pattern)
            .map(|(pos, _)| pos as u64)
    }

    /// Determines whether the toggle `pattern` contains 2, 3, 4, or 7 one-bits,
    /// meaning that the number of lit up segments in the seven-segment digit display
    /// can immediately uniquely identifies 1, 7, 4, or 8, respectively.
    /// An error returned by this function indicates an unrecognizable toggle pattern.
    ///
    /// # Implementation Note
    /// The implementation of this function just straight out decodes any digits,
    /// then just check whether it is one of 1, 4, 7, or 8 after the fact.
    fn can_quickly_decode(&self, pattern: u8) -> anyhow::Result<bool> {
        let digit = self
            .decode_pattern(pattern)
            .with_context(|| format!("cannot decode toggle pattern: {:02b}", pattern))?;
        Ok([1, 4, 7, 8].into_iter().any(|target| digit == target))
    }

    /// Counts the number of toggle patterns within the output displays
    /// that can unique identify a digit solely on the number of one-bits
    /// (i.e. the number of lit up segments in the seven-segment digit display).
    /// Returns an integer from 0 up to 4.
    fn count_quickly_decodable_display_patterns(&self) -> usize {
        self.display_patterns
            .iter()
            .copied()
            .filter(|&p| self.can_quickly_decode(p).unwrap_or(false))
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
            .context("cannot decode output display patterns")
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
            .with_context(|| format!("invalid line display input: {}", s))?;
        let all_patterns: Vec<_> = (1..=14)
            .map(|i| pattern_from_scribbles(&captures[i]))
            .collect::<anyhow::Result<_>>()?;
        let digit_patterns = (&all_patterns[0..10])
            .iter()
            .copied()
            .collect_exact_array()?;
        let display_patterns = (&all_patterns[10..14])
            .iter()
            .copied()
            .collect_exact_array()?;

        Ok(DisplayLog::new(digit_patterns, display_patterns))
    }
}

/// Converts the log scribbles (a string combination of `'a'` through `'g'`) into a toggle pattern,
/// which is an 8-bit integer representation compatible with [`DisplayLog`].
fn pattern_from_scribbles<T: AsRef<str>>(scribbles: T) -> anyhow::Result<u8> {
    let mut pattern = 0;
    for c in scribbles.as_ref().chars() {
        let pos = match c {
            'a'..='g' => c as u32 - 'a' as u32,
            _ => bail!("invalid character: {}", c.escape_default()),
        };
        let mask = 1 << pos;
        ensure!((pattern & mask) == 0, "duplicated character: '{}'", c);
        pattern |= mask;
    }
    Ok(pattern)
}

/// Sorts the toggle patterns so that the `i`-th pattern precisely decodes to digit `i`.
fn sort_toggle_patterns(patterns: &[u8; 10]) -> [u8; 10] {
    let one_decoder = DECODER_BY_NULL_ONE_FOUR[1].0;
    let one_mask = pattern_by_xor_mask_tests(patterns, [(0, one_decoder)].as_slice());
    let four_decoder = DECODER_BY_NULL_ONE_FOUR[4].0;
    let four_mask = pattern_by_xor_mask_tests(patterns, [(0, four_decoder)].as_slice());
    DECODER_BY_NULL_ONE_FOUR.map(|(null, one, four)| {
        let tests = [(0, null), (one_mask, one), (four_mask, four)];
        pattern_by_xor_mask_tests(patterns, tests.as_slice())
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
        .filter(|&n| tests.iter().all(|test| (n ^ test.0).count_ones() == test.1))
        .exactly_one()
        .expect("expected exactly one element here")
}
