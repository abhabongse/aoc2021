//! Day 10: Syntax Scoring, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/10>
use std::io::{BufRead, BufReader};

use clap::Parser;
use lazy_static::lazy_static;

use aoc2021::argparser::Cli;

lazy_static! {
    /// Mapping from closing character to error score
    static ref ERROR_SCORE_BY_CHAR: [(char, i64); 4] = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];
    /// Mapping from closing character to individual autocompletion score
    static ref AUTOCOMPLETE_SCORE_BY_CHAR: [(char, i64); 4] = [(')', 1), (']', 2), ('}', 3), ('>', 4)];
}

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { statements } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Check syntax of all code statements
    let check_results: Vec<_> = statements.iter().map(check_syntax).collect();

    // Part 1: Corrupt error score
    let p1_score: i64 = check_results
        .iter()
        .filter_map(|result| match result {
            SyntaxCheckResult::AutoCompletion(_) => None,
            SyntaxCheckResult::Corrupted(c) => Some(corrupt_error_score(*c)),
        })
        .sum();
    println!("Part 1 answer: {}", p1_score);

    // Part 2: Autocomplete score
    let p2_score = {
        let mut autocomplete_score: Vec<_> = check_results
            .iter()
            .filter_map(|result| match result {
                SyntaxCheckResult::AutoCompletion(s) => Some(autocomplete_score(s)),
                SyntaxCheckResult::Corrupted(_) => None,
            })
            .collect();
        autocomplete_score.sort_unstable();
        autocomplete_score[autocomplete_score.len() / 2]
    };
    println!("Part 2 answer: {}", p2_score);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Submarine system source code
    statements: Vec<String>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut statements = Vec::new();
        for line in reader.lines() {
            statements.push(line?.trim().to_string())
        }
        Ok(Input { statements })
    }
}

/// Possible outcomes for validating a code statement in submarine navigation subsystem
#[derive(Debug, Clone, Eq, PartialEq)]
enum SyntaxCheckResult {
    /// This struct indicates that, when parsing a statement from left to right,
    /// no mismatch between designated pairs of characters have been found.
    /// However, the code may still be incomplete (e.g. hanging open parentheses, brackets, or braces).
    /// In such case, the string would contain the missing closing characters to complete the code statement.
    /// If the original code statement is already complete, the autocomplete string would be empty.
    AutoCompletion(String),
    /// This struct indicates that, when parsing a statement from left to right,
    /// a mismatch between designated pairs of characters has been found.
    /// In such case, it would keep track of the first invalid closing character encountered in the statement.
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

/// Computes the corrupt error score for the given closing character.
fn corrupt_error_score(target: char) -> i64 {
    let mut it = ERROR_SCORE_BY_CHAR.iter().copied();
    let find_result = it.find(|&(c, _v)| c == target).expect("unknown character");
    find_result.1
}

/// Computes the autocomplete score for the given autocompletion string.
fn autocomplete_score<T: AsRef<str>>(s: T) -> i64 {
    s.as_ref().chars().fold(0, |acc, target| {
        let mut it = AUTOCOMPLETE_SCORE_BY_CHAR.iter().copied();
        let find_result = it.find(|&(c, _v)| c == target).expect("unknown character");
        5 * acc + find_result.1
    })
}
