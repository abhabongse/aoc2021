//! Day 5: Hydrothermal Venture, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/5>
use std::io::BufRead;
use std::str::FromStr;

use anyhow::anyhow;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { line_segments } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Axis-aligned line segments only
    let p1_hot_points = {
        let point_covers = line_segments
            .iter()
            .filter(|s| s.is_axis_aligned())
            .flat_map(|s| s.walk_integer_coords())
            .counts();
        point_covers
            .iter()
            .filter_map(|(k, &v)| (v >= 2).then(|| k))
            .count()
    };
    println!("Part 1 answer: {}", p1_hot_points);

    // Part 2: All line segments considered
    let p2_hot_points = {
        let point_covers = line_segments
            .iter()
            .flat_map(|s| s.walk_integer_coords())
            .counts();
        point_covers
            .iter()
            .filter_map(|(k, &v)| (v >= 2).then(|| k))
            .count()
    };
    println!("Part 2 answer: {}", p2_hot_points);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Collection of line segments
    line_segments: Vec<LineSegment>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut line_segments = Vec::new();
        for line in reader.lines() {
            line_segments.push(line?.quickparse()?);
        }
        Ok(Input { line_segments })
    }
}

/// Alias for point in two-dimensional space
type Point = (i64, i64);

/// Line segment with end-point coordinates in two-dimensional space
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct LineSegment {
    /// One end of the line segment
    p: Point,
    /// Another end of the line segment
    q: Point,
}

impl LineSegment {
    /// An iterator which produces a sequence of integer coordinates
    /// contained within the line segment, from point `p` to point `q`.
    fn walk_integer_coords(&self) -> impl Iterator<Item = Point> + '_ {
        let (dx, dy) = (self.q.0 - self.p.0, self.q.1 - self.p.1);
        let steps = num::integer::gcd(dx, dy);

        std::iter::successors(Some(self.p), move |&(x, y)| {
            (self.q != (x, y)).then(|| (x + dx / steps, y + dy / steps))
        })
    }

    /// Checks whethre the line segment is axis-aligned.
    fn is_axis_aligned(&self) -> bool {
        self.p.0 == self.q.0 || self.p.1 == self.q.1
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
            p: (captures[1].parse()?, captures[2].parse()?),
            q: (captures[3].parse()?, captures[4].parse()?),
        })
    }
}
