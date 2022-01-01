//! Day 19: Beacon Scanner, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/19>
use std::fmt::{Debug, Formatter};
use std::io::BufRead;
use std::ops::{Add, Neg, Sub};
use std::str::FromStr;

use anyhow::{ensure, Context};
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::InputSrc;

lazy_static! {
    static ref PERMUTATION_FUNCS: Vec<Box<dyn Fn(Point3D) -> Point3D + Sync>> =
        Point3D::permutation_funcs();
}

/// Main program
fn main() {
    let input_src = InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { reports } = Input::from_buffer(input_reader).expect("cannot parse input");

    let result = reports[0].rotate_and_align(&reports[1], 12, 1000);
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
                reports.push(ScannerReport::new());
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
#[derive(Debug, Clone)]
struct ScannerReport(Vec<Point3D>);

impl ScannerReport {
    /// Creates an empty scanner report.
    fn new() -> Self {
        ScannerReport(Vec::new())
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
            .sorted_by_key(|p| (p.0, p.1, p.2))
            .collect_vec();
        let snd_set = other.0.iter().copied();
        let snd_set = snd_set
            .map(|p| p + offset)
            .filter(|p| p.norm_max() <= scanner_range)
            .sorted_by_key(|p| (p.0, p.1, p.2))
            .collect_vec();
        fst_set == snd_set
    }
}

/// Represents an integer position in 3-dimensional space
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D(i64, i64, i64);

impl Point3D {
    /// Computes the maximum norm of the point.
    fn norm_max(&self) -> i64 {
        [self.0, self.1, self.2]
            .into_iter()
            .map(|v| v.abs())
            .max()
            .unwrap()
    }

    /// Generates all  anonymous functions (i.e. lambdas) which transforms a point
    /// into another position through each of 24 possible cube-rotations.
    fn permutation_funcs() -> Vec<Box<dyn Fn(Point3D) -> Point3D + Sync>> {
        let xyz_rotators = [
            |p: Point3D| p,
            |p: Point3D| Point3D(p.1, p.2, p.0),
            |p: Point3D| Point3D(p.2, p.0, p.1),
        ];
        let xy_reflectors = [|p: Point3D| p, |p: Point3D| Point3D(p.1, p.0, -p.2)];
        let z_rotators = [
            |p: Point3D| p,
            |p: Point3D| Point3D(p.1, -p.0, p.2),
            |p: Point3D| Point3D(-p.0, -p.1, p.2),
            |p: Point3D| Point3D(-p.1, p.0, p.2),
        ];
        iproduct!(xyz_rotators, xy_reflectors, z_rotators)
            .map(|(a, b, c)| {
                Box::new(move |p: Point3D| c(b(a(p)))) as Box<dyn Fn(Point3D) -> Point3D + Sync>
            })
            .collect_vec()
    }
}

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

impl Add for Point3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Neg for Point3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point3D(-self.0, -self.1, -self.2)
    }
}
