//! Day 10: Syntax Scoring, Advent of Code 2021
//! <https://adventofcode.com/2021/day/10>
use std::collections::HashMap;
use std::io::BufRead;

use anyhow::Context;
use lazy_static::lazy_static;

use aoc2021::argparser;

lazy_static! {
    /// Mapping from closing character to error score.
    ///
    /// # Implementation Note
    /// This should have been a simple array of tuples due to smaller overhead.
    static ref ERROR_SCORE_BY_CHAR: HashMap<char, i64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    /// Mapping from closing character to individual autocompletion score.
    ///
    /// # Implementation Note
    /// This should have been a simple array of tuples due to smaller overhead.
    static ref AUTOCOMPLETE_SCORE_BY_CHAR: HashMap<char, i64> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
}

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let lines = parse_input(input_reader).expect("cannot parse input");

    let check_results: Vec<_> = lines.iter().map(check_syntax).collect();

    // Part 1: Corrupt error score.
    let p1_answer: i64 = check_results
        .iter()
        .flat_map(|result| match result {
            SyntaxCheckResult::AutoCompletion(_) => None,
            SyntaxCheckResult::Corrupted(c) => {
                Some(ERROR_SCORE_BY_CHAR.get(c).expect("unknown character"))
            }
        })
        .sum();
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Autocomplete score.
    let mut autocomplete_score: Vec<_> = check_results
        .iter()
        .flat_map(|result| match result {
            SyntaxCheckResult::Corrupted(_) => None,
            SyntaxCheckResult::AutoCompletion(s) => Some(autocomplete_score(s)),
        })
        .collect();
    autocomplete_score.sort_unstable();
    let p2_answer = autocomplete_score[autocomplete_score.len() / 2];
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses the report (program input) as a vector of integers.
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<String>> {
    reader
        .lines()
        .map(|line| {
            Ok(line
                .context("cannot read a line of string")?
                .trim()
                .to_string())
        })
        .collect()
}

/// Three possible outcomes from validating navigating submarine subsystem chunks.
#[derive(Debug, Clone, Eq, PartialEq)]
enum SyntaxCheckResult {
    /// Reading from left to right, no mismatch between designated pairs of characters.
    /// However, the code may still be incomplete.
    /// The autocomplete string contains the missing closing characters to complete the code.
    /// Empty autocomplete string indicates that the original code is already complete.
    AutoCompletion(String),
    /// Reading from left to right, a mismatch between designated pairs of characters has been found.
    /// It keeps track of the first invalid closing character found in the code.
    Corrupted(char),
}

/// Checks the syntax of a line of code from submarine subsystem.
fn check_syntax<T: AsRef<str>>(s: T) -> SyntaxCheckResult {
    let s = s.as_ref();
    let mut stack = Vec::with_capacity(16);
    for c in s.chars() {
        match (stack.last(), c) {
            (_, '(' | '[' | '{' | '<') => stack.push(c),
            (Some(&'('), ')') | (Some(&'['), ']') | (Some(&'{'), '}') | (Some(&'<'), '>') => {
                stack.pop();
            }
            _ => return SyntaxCheckResult::Corrupted(c),
        };
    }
    let auto_completion: String = stack
        .into_iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("this character should never appear in the stack: {}", c),
        })
        .collect();
    SyntaxCheckResult::AutoCompletion(auto_completion)
}

/// Computes the autocomplete score for the given autocompletion string.
fn autocomplete_score<T: AsRef<str>>(s: T) -> i64 {
    s.as_ref().chars().fold(0, |acc, c| {
        let char_score = AUTOCOMPLETE_SCORE_BY_CHAR
            .get(&c)
            .expect("unknown character");
        5 * acc + char_score
    })
}
