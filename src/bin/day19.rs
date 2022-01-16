//! Day 19: Beacon Scanner, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/19>
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, bail, ensure, Context};
use clap::Parser;
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use num::Zero;
use regex::Regex;

use aoc2021::argparser::Cli;
use aoc2021::collect_array::CollectArray;
use aoc2021::vecmat::{CMatrix, CVector};

/// Represents a point in 3-dimensional space
type VecPoint = CVector<i64, 3>;
type TransMatrix = CMatrix<i64, 3, 3>;

lazy_static! {
    static ref CUBE_ROTATIONS: [TransMatrix; 24] = cube_rotations();
}

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { reports } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Reconstruct the locations of scanners and beacons
    // using the orient and align technique, targeting 12 overlapping beacons
    let genesis_report = OrientAlignResult {
        offset: VecPoint::zero(),
        report: reports[0].clone(),
    };
    let mut base_report_queue = VecDeque::from([genesis_report]);
    let mut remaining = reports[1..].iter().cloned().collect_vec();
    let mut beacons = HashSet::new();
    let mut scanners = Vec::new();

    // Take a base report from the queue and try to
    // orient and align all other remaining reports if possible
    while let Some(base_report) = base_report_queue.pop_front() {
        let mut next_remaining = Vec::new();
        for report in remaining {
            if let Some(result) = base_report.report.orient_and_align(&report, 12, 1000) {
                base_report_queue.push_back(OrientAlignResult {
                    offset: base_report.offset + result.offset,
                    report: result.report,
                })
            } else {
                next_remaining.push(report);
            }
        }
        let new_beacons = base_report.report.0.into_iter();
        beacons.extend(new_beacons.map(|p| p + base_report.offset));
        scanners.push(base_report.offset);
        remaining = next_remaining;
    }

    // Part 1: Count all beacons
    let p1_answer = beacons.len();
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Furthest pair of scanners
    let p2_answer = iproduct!(scanners.iter().copied(), scanners.iter().copied())
        .map(|(a, b)| (a - b).norm1())
        .max()
        .expect("empty scanner info");
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
    fn parse_point(s: &str) -> Option<anyhow::Result<VecPoint>> {
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
        Some(Ok(CVector::new([x, y, z])))
    }
}

/// Represents a list of beacon positions reported by a scanner
#[derive(Debug, Clone)]
struct Report(Vec<VecPoint>);

impl Report {
    /// Creates an empty scanner report.
    fn new() -> Self {
        Report(Vec::new())
    }

    /// Adds a new report.
    fn push(&mut self, point: VecPoint) {
        self.0.push(point);
    }

    /// Makes a copy of the report by transforming positions of the beacons
    /// using the specified transformation matrix.
    fn rotate_copy(&self, mat: TransMatrix) -> Self {
        Report(self.0.iter().copied().map(|p| mat * p).collect_vec())
    }

    /// Attempts to rotate the `other` scanner report and aligns its reported beacons with _this_ scanner.
    /// See details about other function parameters from [`ScannerReport::align`].
    fn orient_and_align(
        &self,
        other: &Self,
        beacon_target: usize,
        scanner_range: i64,
    ) -> Option<OrientAlignResult> {
        for mat in CUBE_ROTATIONS.iter().copied() {
            let modified_other = other.rotate_copy(mat);
            if let Some(offset) = self.align(&modified_other, beacon_target, scanner_range) {
                return Some(OrientAlignResult {
                    offset,
                    report: modified_other,
                });
            }
        }
        None
    }

    /// Attempts to align the reports of two scanners over each other
    /// and determines the offset of the `other` scanner in relation to _this_ scanner.
    /// Intersected range of both scanners must see the exact same set of beacons
    /// which must also be at least the specified `beacon_target`.
    /// Function argument `scanner_range` specifies the how far into each direction
    /// that each scanner can see all other beacons.
    /// If alignment fails, this function returns `None`.
    fn align(&self, other: &Self, beacon_target: usize, scanner_range: i64) -> Option<VecPoint> {
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
    fn check(&self, other: &Self, offset: VecPoint, scanner_range: i64) -> bool {
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

/// Result from trying to rotate and align one report over another
#[derive(Debug, Clone)]
struct OrientAlignResult {
    /// Offset of the second scanner from the first scanner
    offset: VecPoint,
    /// Report from the second scanner in the same orientation of the first scanner
    report: Report,
}

/// Generates all transformation matrix which would rotate
/// an axis-aligned cube centered at the origin in all 24 possible ways.
fn cube_rotations() -> [CMatrix<i64, 3, 3>; 24] {
    let xyz_rotate_suite = TransMatrix::xyz_rotate_suite();
    let xy_rotate_suite = TransMatrix::xy_rotate_suite();
    let z_rotate_suite = TransMatrix::z_rotate_suite();
    iproduct!(
        xyz_rotate_suite.iter().copied(),
        xy_rotate_suite.iter().copied(),
        z_rotate_suite.iter().copied()
    )
    .map(|(a, b, c)| c * b * a)
    .collect_exact()
    .unwrap()
}
