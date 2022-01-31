//! Day 22: Reactor Reboot, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/22>
use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;

use anyhow::{bail, Context};
use clap::Parser;
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::Cli;
use aoc2021::parsing::QuickParse;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { reboot_steps } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Cubes within (-50..50)^3
    let region = Cuboid {
        x: Interval::new(-50, 50),
        y: Interval::new(-50, 50),
        z: Interval::new(-50, 50),
    };
    let p1_answer = on_cubes_in_small_cuboid(reboot_steps.as_slice(), &region);
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = on_cubes(reboot_steps.as_slice());
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    reboot_steps: Vec<RebootStep>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut reboot_steps = Vec::new();
        for line in reader.lines() {
            reboot_steps.push(line?.quickparse()?);
        }
        Ok(Input { reboot_steps })
    }
}

/// A step to reboot a reactor
#[derive(Debug, Clone)]
struct RebootStep {
    cuboid: Cuboid,
    state: State,
}

impl FromStr for RebootStep {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                    \s*(on|off)\s+
                    x=(-?\d+)\.\.(-?\d+),
                    y=(-?\d+)\.\.(-?\d+),
                    z=(-?\d+)\.\.(-?\d+)\s*"
            )
            .unwrap();
        }
        let captures = RE
            .captures(s)
            .with_context(|| format!("invalid line input: {}", s))?;
        let cuboid = Cuboid {
            x: Interval::new(captures[2].parse()?, captures[3].parse()?),
            y: Interval::new(captures[4].parse()?, captures[5].parse()?),
            z: Interval::new(captures[6].parse()?, captures[7].parse()?),
        };
        let state = captures[1].parse()?;
        Ok(RebootStep { cuboid, state })
    }
}

/// An item type which can be checked whether it is contained
/// within another container of type `T`
trait Resident<T> {
    /// Checks whether the particle item is contained within the `container`
    fn within(&self, container: &T) -> bool;
}

/// Three-dimensional bounded region
#[derive(Debug, Clone)]
struct Cuboid {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Cuboid {
    /// Volume of the cuboid
    fn volume(&self) -> i64 {
        self.x.len() * self.y.len() * self.z.len()
    }
}

impl Resident<Cuboid> for (i64, i64, i64) {
    fn within(&self, container: &Cuboid) -> bool {
        let (x, y, z) = *self;
        x.within(&container.x) && y.within(&container.y) && z.within(&container.z)
    }
}

impl Resident<Cuboid> for Cuboid {
    fn within(&self, container: &Cuboid) -> bool {
        self.x.within(&container.x) && self.y.within(&container.y) && self.z.within(&container.z)
    }
}

/// Bounded integer interval from the start to just before the end
#[derive(Debug, Clone)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    /// Creates an integer interval, inclusive on lower and upper bounds
    fn new(lower: i64, upper: i64) -> Self {
        assert!(lower <= upper);
        Interval {
            start: lower,
            end: upper + 1,
        }
    }

    /// As [`Range`](std::ops::Range) object
    fn range(&self) -> Range<i64> {
        self.start..self.end
    }

    /// Length of the interval
    fn len(&self) -> i64 {
        self.end - self.start
    }
}

impl Resident<Interval> for i64 {
    fn within(&self, container: &Interval) -> bool {
        container.start <= *self && *self < container.end
    }
}

impl Resident<Interval> for Interval {
    fn within(&self, container: &Interval) -> bool {
        container.start <= self.start && self.end <= container.end
    }
}

/// Target cube state to switch to
#[derive(Debug, Clone)]
enum State {
    On,
    Off,
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(State::On),
            "off" => Ok(State::Off),
            _ => bail!("invalid cube state"),
        }
    }
}

/// Counts the number of on cubes within a small cuboid region
fn on_cubes_in_small_cuboid(reboot_steps: &[RebootStep], region: &Cuboid) -> i64 {
    let x_range = region.x.range();
    let y_range = region.y.range();
    let z_range = region.z.range();
    iproduct!(x_range, y_range, z_range)
        .map(|p| {
            let state = reboot_steps
                .iter()
                .rev()
                .find(|s| p.within(&s.cuboid))
                .map_or(State::Off, |s| s.state.clone());
            match state {
                State::On => 1,
                State::Off => 0,
            }
        })
        .sum()
}

/// Properly counts the number of on cubes
fn on_cubes(reboot_steps: &[RebootStep]) -> i64 {
    let x_intervals =
        IntervalByCoords::intersect_from_intervals(reboot_steps.iter().map(|s| s.cuboid.x.clone()));
    let y_intervals =
        IntervalByCoords::intersect_from_intervals(reboot_steps.iter().map(|s| s.cuboid.y.clone()));
    let z_intervals =
        IntervalByCoords::intersect_from_intervals(reboot_steps.iter().map(|s| s.cuboid.z.clone()));

    iproduct!(x_intervals, y_intervals, z_intervals)
        .map(|(x, y, z)| {
            let cuboid = Cuboid { x, y, z };
            let state = reboot_steps
                .iter()
                .rev()
                .find(|s| cuboid.within(&s.cuboid))
                .map_or(State::Off, |s| s.state.clone());
            match state {
                State::On => cuboid.volume(),
                State::Off => 0,
            }
        })
        .sum()
}

/// Iterator of consecutive intervals whose endpoints are described by
/// monotonically increasing integer coordinates
#[derive(Debug, Clone)]
struct IntervalByCoords {
    coords: Vec<i64>,
    counter: usize,
}

impl IntervalByCoords {
    /// A new iterator of intervals created by intersecting a collection of intervals with each other
    fn intersect_from_intervals(intervals: impl Iterator<Item = Interval>) -> Self {
        let mut coords = BTreeSet::new();
        for intv in intervals {
            coords.insert(intv.start);
            coords.insert(intv.end);
        }
        let coords = coords.into_iter().dedup().collect();
        IntervalByCoords { coords, counter: 0 }
    }
}

impl Iterator for IntervalByCoords {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter + 1 >= self.coords.len() {
            None
        } else {
            let intv = Interval {
                start: self.coords[self.counter],
                end: self.coords[self.counter + 1],
            };
            self.counter += 1;
            Some(intv)
        }
    }
}
