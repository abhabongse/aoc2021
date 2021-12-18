//! Day 14: Extended Polymerization, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/14>
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

use anyhow::{bail, Context};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;
use aoc2021::try_collect::TryCollectArray;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input {
        template,
        ins_rules,
    } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Save first and last chars for reconciliation later
    let first = template.chars().next().expect("empty template string");
    let last = template.chars().last().unwrap();

    // Count bigrams or the original template polymer
    let bigram_counts = template.chars().tuple_windows::<(_, _)>().counts();

    // Part 1: Applying insertion rules 10 times
    let bigram_counts = (0..10).fold(bigram_counts, |counts, _| {
        next_polymer_bigram_counts(&counts, ins_rules.as_slice())
    });
    let p1_diff = {
        let unigram_counts = unigrams_from_bigrams(first, last, &bigram_counts);
        let (&min_count, &max_count) = unigram_counts.values().minmax().into_option().unwrap();
        max_count - min_count
    };
    println!("Part 1 answer: {}", p1_diff);

    // Part 2: Apply insertion rules 30 more times
    let bigram_counts = (0..30).fold(bigram_counts, |counts, _| {
        next_polymer_bigram_counts(&counts, ins_rules.as_slice())
    });
    let p2_diff = {
        let unigram_counts = unigrams_from_bigrams(first, last, &bigram_counts);
        let (&min_count, &max_count) = unigram_counts.values().minmax().into_option().unwrap();
        max_count - min_count
    };
    println!("Part 2 answer: {}", p2_diff);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Starting polymer template as a sequence of characters
    template: String,
    /// Collection of insertion rules
    ins_rules: Vec<InsertionRule>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let template = match lines.next() {
            Some(Ok(s)) => s.trim().to_string(),
            _ => bail!("expected the template string on the first line"),
        };
        match lines.next() {
            Some(Ok(s)) if s.trim().is_empty() => {}
            _ => bail!("expected an empty line after the first line"),
        }
        let mut ins_rules = Vec::new();
        for line in lines {
            let line = line.context("cannot read a line of string")?;
            ins_rules.push(line.quickparse()?);
        }
        Ok(Input {
            template,
            ins_rules,
        })
    }
}

/// Bigram: a consecutive pair of characters
type Bigram = (char, char);

/// Polymerization insertion rules
#[derive(Debug, Clone)]
struct InsertionRule {
    /// Pair of characters to capture
    pattern: Bigram,
    /// Character to insert between the pair of pattern characters
    insert_char: char,
}

impl FromStr for InsertionRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s*(\w\w)\s+->\s+(\w)\s*").unwrap();
        }
        // let RE: Regex = Regex::new(r"\s*(\w\w)\s+->\s+(\w)\s*").unwrap();
        let captures = RE
            .captures(s)
            .with_context(|| format!("invalid insertion rule: {}", s))?;
        let [fst, snd] = captures[1].chars().try_collect_exact_array()?;
        let [insert_char] = captures[2].chars().try_collect_exact_array()?;
        let pattern = (fst, snd);
        Ok(InsertionRule {
            pattern,
            insert_char,
        })
    }
}

/// Computes the bigram counts of the next polymer obtained by transforming the input polymer
/// (whose bigram counts is given as input) based on `insertion_rules`.
fn next_polymer_bigram_counts(
    bigram_counts: &HashMap<Bigram, usize>,
    insertion_rules: &[InsertionRule],
) -> HashMap<Bigram, usize> {
    let mut next_bigram_counts = HashMap::new();
    for rule in insertion_rules {
        let count = bigram_counts.get(&rule.pattern).copied().unwrap_or(0);
        if count == 0 {
            continue;
        }
        *next_bigram_counts
            .entry((rule.pattern.0, rule.insert_char))
            .or_insert(0) += count;
        *next_bigram_counts
            .entry((rule.insert_char, rule.pattern.1))
            .or_insert(0) += count;
    }
    next_bigram_counts
}

/// Counts individual elements based on bigram counts of a polymer.
fn unigrams_from_bigrams(
    first: char,
    last: char,
    bigram_counts: &HashMap<Bigram, usize>,
) -> HashMap<char, usize> {
    let mut unigram_counts = HashMap::from([(first, 1), (last, 1)]);
    for (bigram, count) in bigram_counts.iter() {
        *unigram_counts.entry(bigram.0).or_insert(0) += count;
        *unigram_counts.entry(bigram.1).or_insert(0) += count;
    }
    // Undo double counting
    unigram_counts.values_mut().for_each(|v| *v /= 2);
    unigram_counts
}
