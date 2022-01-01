//! Day 19: Beacon Scanner, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/19>
use std::fmt::Debug;
use std::io::BufRead;

use anyhow::{anyhow, bail, ensure, Context};
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::InputSrc;
use aoc2021::collect_array::CollectArray;
use aoc2021::vecmat::{CardinalMatrix, CardinalVector};

lazy_static! {
    static ref CUBE_ROTATIONS: [CardinalMatrix<i64, 3, 3>; 24] = cube_rotations();
}

/// Main program
fn main() {
    let input_src = InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { reports } = Input::from_buffer(input_reader).expect("cannot parse input");

    let p = CardinalVector::new([1, 2, 3]);
    for mat in CUBE_ROTATIONS.iter().copied() {
        eprintln!("{:?}", mat * p);
    }

    let result = reports[0].rotate_and_align(&reports[1], 3, 1000);
    eprintln!("{:?}", result);

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
    reports: Vec<Report>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut reports = Vec::new();
        for line in reader.lines() {
            let line = line.context("cannot read a line")?;
            if line.trim().is_empty() {
                continue;
            } else if let Some(id_result) = Input::parse_scanner_header(line.as_str()) {
                let id = id_result?;
                ensure!(
                    id == reports.len(),
                    "invalid scanner id: {} but expected {}",
                    id,
                    reports.len()
                );
                reports.push(Report::new());
            } else if let Some(point_result) = Input::parse_point(line.as_str()) {
                let point = point_result?;
                reports
                    .last_mut()
                    .with_context(|| {
                        format!(
                            "not started with scanner header: '{}'",
                            line.escape_default()
                        )
                    })?
                    .push(point);
            } else {
                bail!("unrecognized line format: '{}'", line.escape_default());
            }
        }
        Ok(Input { reports })
    }

    /// Attempts to parse a scanner header line for the scanner id.
    /// `None` is returned if the line format does not match.
    /// Other kinds of parsing errors will result in `Some(Err(anyhow::Error))`.
    fn parse_scanner_header(s: &str) -> Option<anyhow::Result<usize>> {
        lazy_static! {
            static ref SCANNER_HEADER_RE: Regex =
                Regex::new(r"(?i)\s*-+\s*scanner\s+(\d+)\s*-+\s*").unwrap();
        }
        let captures = SCANNER_HEADER_RE.captures(s)?;
        Some(captures[1].parse().with_context(|| {
            format!(
                "cannot parse scanner id: '{}'",
                captures[1].escape_default()
            )
        }))
    }

    /// Attempts to parse a comma-seperated data into a point in 3-dimensional space.
    /// `None` is returned if the line format does not match.
    /// Other kinds of parsing errors will result in `Some(Err(anyhow::Error))`.
    fn parse_point(s: &str) -> Option<anyhow::Result<Point3D>> {
        lazy_static! {
            static ref COORDS_RE: Regex = Regex::new(
                r"(?x)
                    \s*(-?\d+)\s*,
                    \s*(-?\d+)\s*,
                    \s*(-?\d+)\s*",
            )
            .unwrap();
        }
        let captures = COORDS_RE.captures(s)?;
        let x = match captures[1].parse() {
            Ok(value) => value,
            _ => return Some(Err(anyhow!("cannot parse integer"))),
        };
        let y = match captures[2].parse() {
            Ok(value) => value,
            _ => return Some(Err(anyhow!("cannot parse integer"))),
        };
        let z = match captures[3].parse() {
            Ok(value) => value,
            _ => return Some(Err(anyhow!("cannot parse integer"))),
        };
        Some(Ok(CardinalVector::new([x, y, z])))
    }
}

/// Represents a point in 3-dimensional space
type Point3D = CardinalVector<i64, 3>;
type TransMatrix = CardinalMatrix<i64, 3, 3>;

/// Represents a list of beacon positions reported by a scanner
#[derive(Debug, Clone)]
struct Report(Vec<Point3D>);

impl Report {
    /// Creates an empty scanner report.
    fn new() -> Self {
        Report(Vec::new())
    }

    /// Adds a new report.
    fn push(&mut self, point: Point3D) {
        self.0.push(point);
    }

    /// Attempts to rotate the `other` scanner report and aligns its reported beacons with _this_ scanner.
    /// See details about other function parameters from [`ScannerReport::align`].
    fn rotate_and_align(
        &self,
        other: &Self,
        beacon_target: usize,
        scanner_range: i64,
    ) -> Option<Point3D> {
        let result = self.align(other, beacon_target, scanner_range);
        eprintln!("{:?}", result);
        todo!()
    }

    /// Attempts to align the reports of two scanners over each other
    /// and determines the offset of the `other` scanner in relation to _this_ scanner.
    /// Intersected range of both scanners must see the exact same set of beacons
    /// which must also be at least the specified `beacon_target`.
    /// Function argument `scanner_range` specifies the how far into each direction
    /// that each scanner can see all other beacons.
    /// If alignment fails, this function returns `None`.
    fn align(&self, other: &Self, beacon_target: usize, scanner_range: i64) -> Option<Point3D> {
        let paired_offsets = iproduct!(self.0.iter().copied(), other.0.iter().copied())
            .map(|(dp, dq)| dp - dq)
            .counts();
        let offset_candidates = paired_offsets
            .into_iter()
            .filter(|(_, count)| *count >= beacon_target)
            .sorted_by_key(|(_, count)| *count)
            .map(|(offset, _)| offset)
            .collect_vec();
        offset_candidates
            .into_iter()
            .find(|offset| self.check(other, *offset, scanner_range))
    }

    /// Checks if the specific alignment `offset` between two scanners works as it should be,
    /// i.e. the number of overlapping beacons reaches the `beacon_target` with the `scanner_range`.
    fn check(&self, other: &Self, offset: Point3D, scanner_range: i64) -> bool {
        let fst_set = self.0.iter().copied();
        let fst_set = fst_set
            .filter(|p| (*p - offset).norm_max() <= scanner_range)
            .sorted_by_key(|p| p.to_vec())
            .collect_vec();
        let snd_set = other.0.iter().copied();
        let snd_set = snd_set
            .map(|p| p + offset)
            .filter(|p| p.norm_max() <= scanner_range)
            .sorted_by_key(|p| p.to_vec())
            .collect_vec();
        fst_set == snd_set
    }
}

/// Generates all transformation matrix which would rotate
/// an axis-aligned cube centered at the origin in all 24 possible ways.
fn cube_rotations() -> [CardinalMatrix<i64, 3, 3>; 24] {
    let xyz_rotate_suite = TransMatrix::xyz_rotate_suite();
    let xy_rotate_suite = TransMatrix::xy_rotate_suite();
    let z_rotate_suite = TransMatrix::z_rotate_suite();
    iproduct!(
        xyz_rotate_suite.iter().copied(),
        xy_rotate_suite.iter().copied(),
        z_rotate_suite.iter().copied()
    )
    .map(|(a, b, c)| c * b * a)
    .collect_exact_array()
    .unwrap()
}
