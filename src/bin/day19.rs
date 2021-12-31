//! Day 19: Beacon Scanner, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/19>
use std::collections::HashMap;
use std::io::BufRead;

use anyhow::Context;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::InputSrc;

/// Main program
fn main() {
    let input_src = InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { reports } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: TODO
    let p1_answer = 0;
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    reports: Vec<ScannerReport>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        // lazy_static! {
        let SCANNER_HEADER_RE: Regex = Regex::new(r"(?i)\s*-+\s*scanner\s+(\d+)\s*-+\s*").unwrap();
        let COORDS_RE: Regex = Regex::new(
            r"(?x)
                    \s*(-?\d+)\s*,
                    \s*(-?\d+)\s*,
                    \s*(-?\d+)\s*",
        )
        .unwrap();
        // }
        let mut reports: HashMap<String, ScannerReport> = HashMap::new();
        for line in reader.lines() {
            let line = line.context("cannot read a line")?;
            if let Some(captures) = SCANNER_HEADER_RE.captures(line.as_str()) {
            } else if let Some(captures) = COORDS_RE.captures(line.as_str()) {
            } else {
            }
        }
        let reports = reports.into_values().collect_vec();
        Ok(Input { reports })
    }
}

/// Represents a report of a scanner
type ScannerReport = Vec<Point3D>;

/// Represents an integer position in 3-dimensional space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point3D(i64, i64, i64);
