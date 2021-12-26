//! Day 17: Trick Shot, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/17>
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::ops::Neg;

use anyhow::{bail, Context};
use lazy_static::lazy_static;
use num::{PrimInt, Signed};
use regex::Regex;

use aoc2021::argparser;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { target_x, target_y } =
        Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Highest point while hitting the testing range
    let p1_answer = {
        let (_, vy) = test_highest_probe(target_x, target_y);
        peak_distance(vy.unwrap())
    };
    println!("Part 1 answer: {:?}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    target_x: IntRange<i64>,
    target_y: IntRange<i64>,
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
        let target_x = IntRange::new(captures[1].parse()?, captures[2].parse()?)?;
        let target_y = IntRange::new(captures[3].parse()?, captures[4].parse()?)?;
        Ok(Input { target_x, target_y })
    }
}

/// Represents the range of integer coordinates on an axis.
#[derive(Debug, Clone, Copy)]
struct IntRange<T> {
    /// Inclusive lower bound of the range
    lower: T,
    /// Inclusive upper bound of the range
    upper: T,
}

impl<T> IntRange<T> {
    /// Constructs a new integer range
    fn new(lower: T, upper: T) -> anyhow::Result<Self>
    where
        T: PrimInt + Display,
    {
        if lower > upper {
            bail!("conflicting range parameter: {} > {}", lower, upper);
        }
        Ok(IntRange { lower, upper })
    }

    /// Checks whether the given point lies within the range.
    fn contains(&self, point: T) -> bool
    where
        T: PrimInt,
    {
        self.lower <= point && point <= self.upper
    }
}

impl<T> Neg for IntRange<T>
where
    T: Signed,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        IntRange {
            lower: -self.upper,
            upper: -self.lower,
        }
    }
}

impl<T> Display for IntRange<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.lower, self.upper)
    }
}

/// Performs multiple probing test: finds the starting velocity that would hit the target range
/// as well as vertically reaching the highest point possible.
/// If the optimal velocity is unbounded, then `None` is returned.
fn test_highest_probe(target_x: IntRange<i64>, target_y: IntRange<i64>) -> (i64, Option<i64>) {
    // Make sure that target x-range lies inside positive half-space
    let (sign_x, target_x) = match target_x {
        IntRange { lower, upper: _ } if lower > 0 => (1, target_x),
        IntRange { lower: _, upper } if upper < 0 => (-1, -target_x),
        _ => {
            // Special case: if the target x-range overlaps the line `x = 0`
            // then we only need to worry about y-velocity
            return (0, max_vertical_velocity_to_hit(target_y));
        }
    };

    let vx_range = {
        let vx_min = min_velocity_to_reach(target_x.lower);
        let vx_max = target_x.upper;
        vx_min..=vx_max
    };

    let vy_range = match target_y {
        // Case 1: Target is definitely above the level `y = 0`
        IntRange { lower, upper } if lower > 0 => {
            let vy_min = min_velocity_to_reach(lower);
            let vy_max = upper;
            vy_min..=vy_max
        }
        // Case 2: Target is definitely below the level `y = 0`
        IntRange { lower, upper } if upper < 0 => {
            let vy_min = lower;
            let vy_max = lower.abs() - 1;
            vy_min..=vy_max
        }
        // Case 3: Target overlaps the level `y = 0`
        IntRange { lower, upper } => {
            // Make sure to rule out cases when vertical velocity is unbounded right away
            let vx = min_velocity_to_reach(target_x.lower);
            if target_x.contains(peak_distance(vx)) {
                return (sign_x * vx, None);
            }
            let vy_min = lower;
            let vy_max = i64::max(upper, lower.abs() - 1);
            vy_min..=vy_max
        }
    };

    println!("vx in {:?}, vy in {:?}", vx_range, vy_range);
    todo!()
}

/// Maximum vertical (y-) velocity that would hit the target range.
/// If the target range contains the point `y = 0` then any unbounded velocity would work,
/// and thus `None` would be returned in such case.
fn max_vertical_velocity_to_hit(target: IntRange<i64>) -> Option<i64> {
    match target {
        IntRange { lower, upper } if lower > 0 => Some(upper),
        IntRange { lower, upper } if upper < 0 => Some(lower.abs() - 1),
        _ => None,
    }
}

/// Minimum velocity required to at least reach a certain (positive) distance.
fn min_velocity_to_reach(dist: i64) -> i64 {
    assert!(dist >= 0);
    ((2.0 * (dist as f64) + 0.25).sqrt() - 0.5).ceil() as i64
}

/// Peak distance from the given starting velocity.
fn peak_distance(start_velocity: i64) -> i64 {
    assert!(start_velocity >= 0);
    start_velocity * (start_velocity + 1) / 2
}
