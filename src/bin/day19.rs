//! Day 19: Beacon Scanner, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/19>
use std::fmt::{Debug, Formatter};
use std::io::BufRead;
use std::str::FromStr;

use anyhow::{ensure, Context};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::InputSrc;

/// Main program
fn main() {
    let input_src = InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { reports } = Input::from_buffer(input_reader).expect("cannot parse input");

    for (i, r) in reports.iter().enumerate() {
        eprintln!("=== scanner {} ===", i);
        eprintln!("{:?}", r);
    }

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
        lazy_static! {
            static ref SCANNER_HEADER_RE: Regex =
                Regex::new(r"(?i)\s*-+\s*scanner\s+(\d+)\s*-+\s*").unwrap();
        }
        let mut reports = Vec::new();
        for line in reader.lines() {
            let line = line.context("cannot read a line")?;
            if line.trim().is_empty() {
                continue;
            } else if let Some(captures) = SCANNER_HEADER_RE.captures(line.as_str()) {
                let id: usize = captures[1].parse()?;
                ensure!(
                    id == reports.len(),
                    "invalid scanner id: {} but expected {}",
                    id,
                    reports.len()
                );
                reports.push(Vec::new());
            } else {
                let current_report = reports.last_mut().with_context(|| {
                    format!(
                        "not started with scanner header: '{}'",
                        line.escape_default()
                    )
                })?;
                current_report.push(line.parse()?)
            }
        }
        Ok(Input { reports })
    }
}

/// Represents a report of a scanner
type ScannerReport = Vec<Point3D>;

/// Represents an integer position in 3-dimensional space
#[derive(Clone, Copy, PartialEq, Eq)]
struct Point3D(i64, i64, i64);

impl FromStr for Point3D {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref COORDS_RE: Regex = Regex::new(
                r"(?x)
                    \s*(-?\d+)\s*,
                    \s*(-?\d+)\s*,
                    \s*(-?\d+)\s*"
            )
            .unwrap();
        }
        let captures = COORDS_RE
            .captures(s)
            .with_context(|| format!("invalid point format: '{}'", s.escape_default()))?;
        Ok(Point3D(
            captures[1].parse()?,
            captures[2].parse()?,
            captures[3].parse()?,
        ))
    }
}

impl Debug for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}
