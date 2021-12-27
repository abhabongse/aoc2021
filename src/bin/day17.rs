//! Day 17: Trick Shot, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/17>
use std::fmt::Display;
use std::io::BufRead;
use std::ops::RangeInclusive;

use anyhow::{ensure, Context};
use itertools::iproduct;
use lazy_static::lazy_static;
use num::PrimInt;
use regex::Regex;

use aoc2021::argparser;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { target } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Obtains the feasible velocities for the probe to be able to hit the target
    let velocity_ranges = feasible_velocities(target).expect("unbounded feasible velocities");

    // Part 1: Highest point while hitting the testing range
    let p1_answer = {
        let (_, vy) = solve_highest_peak(target, velocity_ranges);
        peak_distance(vy)
    };
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Count all possible trajectories
    let p2_answer = {
        let (vx_range, vy_range) = velocity_ranges.as_range_inclusive();
        iproduct!(vx_range, vy_range)
            .filter(|&(vx, vy)| test_simulate(target, vx, vy))
            .count()
    };
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    target: Rect<i64>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                    \s*target\s+area:
                    \s*x=(-?\d+)..(-?\d+)\s*,
                    \s*y=(-?\d+)..(-?\d+)\s*"
            )
            .unwrap();
        }
        let line = reader.lines().next().context("missing first line")??;
        let captures = RE
            .captures(line.as_str())
            .with_context(|| format!("invalid line input: {}", line))?;
        let target = Rect::new(
            captures[4].parse()?,
            captures[2].parse()?,
            captures[3].parse()?,
            captures[1].parse()?,
        )?;
        Ok(Input { target })
    }
}

/// Represents a bounded rectangular area
#[derive(Debug, Clone, Copy)]
struct Rect<T> {
    /// Inclusive lower bound on (horizontal) x-value
    horz_lower: T,
    /// Inclusive upper bound on (horizontal) x-value
    horz_upper: T,
    /// Inclusive lower bound on (vertical) y-value
    vert_lower: T,
    /// Inclusive upper bound on (vertical) y-value
    vert_upper: T,
}

impl<T> Rect<T> {
    /// Construct a new bounded rectangular area
    fn new(vert_upper: T, horz_upper: T, vert_lower: T, horz_lower: T) -> anyhow::Result<Self>
    where
        T: PrimInt + Display,
    {
        ensure!(
            horz_lower <= horz_upper,
            "conflicting horizontal range: {} > {}",
            horz_lower,
            horz_upper,
        );
        ensure!(
            vert_lower < vert_upper,
            "conflicting vertical range: {} > {}",
            vert_lower,
            vert_upper,
        );
        Ok(Rect {
            horz_lower,
            horz_upper,
            vert_lower,
            vert_upper,
        })
    }

    /// Obtains the rectangle area as a pair of horizontal and vertical [`RangeInclusive`]
    fn as_range_inclusive(&self) -> (RangeInclusive<T>, RangeInclusive<T>)
    where
        T: PrimInt,
    {
        let horz_range = RangeInclusive::new(self.horz_lower, self.horz_upper);
        let vert_range = RangeInclusive::new(self.vert_lower, self.vert_upper);
        (horz_range, vert_range)
    }

    /// Whether the rectangle area contains the given point
    fn contains(&self, x: T, y: T) -> bool
    where
        T: PrimInt,
    {
        self.horz_lower <= x && x <= self.horz_upper && self.vert_lower <= y && y <= self.vert_upper
    }
}

/// Calculates the tight bound for integer-value, feasible starting velocities
/// for the probe which would eventually hit the specified rectangular target.
/// Bounds for horizontal and vertical velocities are determined independently.
/// If the solution is unbounded, then this function would return `None` instead.
fn feasible_velocities(target: Rect<i64>) -> Option<Rect<i64>> {
    // Compute the tight bound for x-velocity search space
    let (vx_lower, vx_upper) = if target.horz_lower > 0 {
        (min_velocity_to_reach(target.horz_lower), target.horz_upper)
    } else if target.horz_upper < 0 {
        (
            target.horz_lower,
            -min_velocity_to_reach(-target.horz_upper),
        )
    } else {
        (target.horz_lower, target.horz_upper)
    };

    // Compute the tight bound for y-velocity search space
    let (vy_lower, vy_upper) = if target.vert_lower > 0 {
        // Case 1: Target rectangle is above the level `y = 0`
        (min_velocity_to_reach(target.vert_lower), target.vert_upper)
    } else if target.vert_upper < 0 {
        // Case 2: Target rectangle is below the level `y = 0`
        (target.vert_lower, target.vert_lower.abs() - 1)
    } else {
        // Case 3: Target rectangle overlaps the level `y = 0`
        // Hence, we need to rule out sub-cases where y-velocity can be unbounded
        if target.horz_lower <= 0 && 0 <= target.horz_upper {
            return None;
        }
        let x_abs_lower = i64::min(target.horz_lower.abs(), target.horz_upper.abs());
        let x_abs_upper = i64::max(target.horz_lower.abs(), target.horz_upper.abs());
        if peak_distance(min_velocity_to_reach(x_abs_lower)) <= x_abs_upper {
            return None;
        }
        let vy_upper = i64::max(target.horz_upper, target.horz_lower.abs() - 1);
        (target.horz_lower, vy_upper)
    };

    Some(Rect::new(vy_upper, vx_upper, vy_lower, vx_lower).unwrap())
}

/// Minimum velocity required to at least reach a certain (positive) distance.
/// This functions provides a tighter lower bound for velocity search space than just velocity 0.
fn min_velocity_to_reach(dist: i64) -> i64 {
    assert!(dist >= 0);
    ((2.0 * (dist as f64) + 0.25).sqrt() - 0.5).ceil() as i64
}

/// Peak distance from the given starting velocity.
fn peak_distance(start_velocity: i64) -> i64 {
    assert!(start_velocity >= 0);
    start_velocity * (start_velocity + 1) / 2
}

/// Finds a starting velocity within the feasible bound that would lead to the probe hitting the target
/// while also reaching the highest peak vertically possible.
fn solve_highest_peak(target: Rect<i64>, velocity_range: Rect<i64>) -> (i64, i64) {
    let (vx_range, vy_range) = velocity_range.as_range_inclusive();
    for (vy, vx) in iproduct!(vy_range.rev(), vx_range.rev()) {
        if test_simulate(target, vx, vy) {
            return (vx, vy);
        }
    }
    panic!("no feasible velocity")
}

/// Runs the simulation to see whether the provided x- and y-velocity
/// would make the probe hit the target within the specified range.
fn test_simulate(target: Rect<i64>, mut vx: i64, mut vy: i64) -> bool {
    let mut x = 0;
    let mut y = 0;
    while vy >= 0 || y > target.vert_lower {
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
        if target.contains(x, y) {
            return true;
        }
    }
    false
}
