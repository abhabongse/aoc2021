//! Day 2: Dive!, Advent of Code 2021
//! https://adventofcode.com/2021/day/2
use std::io::BufRead;
use std::str::FromStr;

use anyhow::anyhow;
use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use regex::Regex;

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let input = parse_input(input_reader).expect("cannot parse input");

    let axis_aligned_segments = input.iter().filter(|s| s.is_axis_aligned());
    let p1_point_covers = axis_aligned_segments
        .map(|s| s.walk_integer_coords())
        .flatten()
        .counts();
    let p1_hot_points = p1_point_covers
        .iter()
        .filter_map(|(k, v)| (*v >= 2).then(|| k));
    println!("Part 1 answer: {}", p1_hot_points.count());

    let p2_point_covers = input
        .iter()
        .map(|s| s.walk_integer_coords())
        .flatten()
        .counts();
    let p2_answer = p2_point_covers
        .iter()
        .filter_map(|(k, v)| (*v >= 2).then(|| k));
    println!("Part 2 answer: {}", p2_answer.count());
}

fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<LineSegment>> {
    reader.lines().map(|line| line?.parse()).collect()
}

/// Two-dimensional Geometric line segment with integer end point coordinates.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct LineSegment {
    /// One end point of the line segment.
    p: Vector2<isize>,
    /// Another end point of the line segment.
    q: Vector2<isize>,
}

impl LineSegment {
    /// Returns an iterator which produces a sequence of integer coordinates
    /// contained within the line segment from point `p` to `q`.
    fn walk_integer_coords(&self) -> impl Iterator<Item = Vector2<isize>> + '_ {
        let diff = self.q - self.p;
        let (dx, dy) = (diff.x, diff.y);
        let steps = num::integer::gcd(dx, dy);

        std::iter::successors(Some(self.p), move |p| {
            if *p == self.q {
                None
            } else {
                Some(p + diff / steps)
            }
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
            static ref RE: Regex =
                Regex::new(r"\s*(-?\d+)\s*,\s*(-?\d+)\s*->\s*(-?\d+)\s*,\s*(-?\d+)\s*").unwrap();
        }
        let captures = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid line segment input: {}", s))?;
        Ok(LineSegment {
            p: Vector2::new(
                captures.get(1).unwrap().as_str().parse()?,
                captures.get(2).unwrap().as_str().parse()?,
            ),
            q: Vector2::new(
                captures.get(3).unwrap().as_str().parse()?,
                captures.get(4).unwrap().as_str().parse()?,
            ),
        })
    }
}
