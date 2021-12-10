//! Day 5: Hydrothermal Venture, Advent of Code 2021
//! https://adventofcode.com/2021/day/5
use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::SVector;
use regex::Regex;

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let line_segments = parse_input(input_reader).expect("cannot parse input");

    // Part 1: axis-aligned line segments only
    let p1_point_covers = line_segments
        .iter()
        .filter(|s| s.is_axis_aligned())
        .map(|s| s.walk_integer_coords())
        .flatten()
        .counts();
    let p1_hot_points = p1_point_covers
        .iter()
        .filter_map(|(k, v)| (*v >= 2).then(|| k))
        .count();
    println!("Part 1 answer: {}", p1_hot_points);

    // Part 2: all line segments considered
    let p2_point_covers = line_segments
        .iter()
        .map(|s| s.walk_integer_coords())
        .flatten()
        .counts();
    let p2_hot_points = p2_point_covers
        .iter()
        .filter_map(|(k, v)| (*v >= 2).then(|| k))
        .count();
    println!("Part 2 answer: {}", p2_hot_points);
}

fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<LineSegment>> {
    reader
        .lines()
        .map(|line| line.context("cannot read a line of string")?.parse())
        .collect()
}

/// Two-dimensional Geometric line segment with integer end point coordinates.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct LineSegment {
    /// One end point of the line segment.
    p: SVector<i64, 2>,
    /// Another end point of the line segment.
    q: SVector<i64, 2>,
}

impl LineSegment {
    /// Returns an iterator which produces a sequence of integer coordinates
    /// contained within the line segment from point `p` to `q`.
    fn walk_integer_coords(&self) -> impl Iterator<Item = SVector<i64, 2>> + '_ {
        let diff = self.q - self.p;
        let (dx, dy) = (diff.x, diff.y);
        let steps = num::integer::gcd(dx, dy);

        std::iter::successors(Some(self.p), move |p| {
            (*p != self.q).then(|| p + diff / steps)
        })
    }

    /// Checks if the line segment is axis-aligned.
    fn is_axis_aligned(&self) -> bool {
        self.p.x == self.q.x || self.p.y == self.q.y
    }
}

impl FromStr for LineSegment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                    \s*(-?\d+)\s*,
                    \s*(-?\d+)\s*->
                    \s*(-?\d+)\s*,
                    \s*(-?\d+)\s*"
            )
            .unwrap();
        }
        let captures = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid line segment input: {}", s))?;
        Ok(LineSegment {
            p: SVector::<_, 2>::new(captures[1].parse()?, captures[2].parse()?),
            q: SVector::<_, 2>::new(captures[3].parse()?, captures[4].parse()?),
        })
    }
}
