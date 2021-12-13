//! Day 13: Transparent Origami, Advent of Code 2021
//! <https://adventofcode.com/2021/day/13>
use std::collections::HashSet;
use std::io;
use std::io::{BufRead, Write};

use anyhow::{anyhow, bail, ensure, Context};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;
use aoc2021::try_collect::TryCollectArray;

use crate::FoldInstr::{XEquals, YEquals};

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { dots, fold_instrs } = parse_input(input_reader).expect("cannot parse input");

    // Part 1: first fold only
    let p1_dot_count = {
        let dots: HashSet<Point> = dots
            .iter()
            .map(|dot| fold_instrs[0].fold_point(*dot))
            .collect();
        dots.len()
    };
    println!("Part 1 answer: {}", p1_dot_count);

    // Part 2: fold and print result
    let dots: HashSet<Point> = fold_instrs
        .iter()
        .fold(dots.into_iter().collect(), |dots, instr| {
            dots.into_iter().map(|dot| instr.fold_point(dot)).collect()
        });
    let mut debug_writer = io::LineWriter::new(io::stdout());
    println!("Part 2 answer: (see below)");
    write_dots(&mut debug_writer, &dots).expect("error while printing dots to stderr");
}

/// Parses dot locations and fold instructions.
fn parse_input<BR: BufRead>(reader: BR) -> anyhow::Result<Input> {
    let mut dots = Vec::new();
    let mut fold_instrs = Vec::new();

    let mut lines = reader.lines();
    for line in lines.by_ref() {
        let line = line.context("cannot read a line of string")?;
        if line.trim().is_empty() {
            break;
        }
        let [x, y] = line.split(',').try_collect_exact_array()?;
        dots.push(Point {
            x: x.trim().quickparse()?,
            y: y.trim().quickparse()?,
        })
    }
    for line in lines.by_ref() {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s*fold\s+along\s+([xy])=(\d+)\s*").unwrap();
        }
        let line = line.context("cannot read a line of string")?;
        let captures = RE
            .captures(line.as_str())
            .ok_or_else(|| anyhow!("invalid folding instruction: {}", line))?;
        let c = captures[2].quickparse()?;
        let instr = match &captures[1] {
            "x" => XEquals(c),
            "y" => YEquals(c),
            axis => bail!("unrecognized axis: {}", axis),
        };
        fold_instrs.push(instr);
    }
    Ok(Input { dots, fold_instrs })
}

/// Represents input data for the problem.
#[derive(Debug, Clone)]
struct Input {
    dots: Vec<Point>,
    fold_instrs: Vec<FoldInstr>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

/// Fold instructions, either along X=c or along Y=c.
#[derive(Debug, Clone, Copy)]
enum FoldInstr {
    XEquals(u64),
    YEquals(u64),
}

impl FoldInstr {
    /// Folds a given dot over the instruction to produce a new dot.
    fn fold_point(&self, dot: Point) -> Point {
        fn fold_around(value: u64, around: u64) -> u64 {
            u64::min(value, 2 * around - value)
        }
        match self {
            FoldInstr::XEquals(c) => Point {
                x: fold_around(dot.x, *c),
                y: dot.y,
            },
            FoldInstr::YEquals(c) => Point {
                x: dot.x,
                y: fold_around(dot.y, *c),
            },
        }
    }
}

/// Printing the dots as the debugging mechanisms
fn write_dots<W: Write>(writer: &mut W, dots: &HashSet<Point>) -> anyhow::Result<()> {
    ensure!(!dots.is_empty(), "empty dots specified");
    let rows = *dots.iter().map(|Point { x: _, y }| y).max().unwrap() + 1;
    let cols = *dots.iter().map(|Point { x, y: _ }| x).max().unwrap() + 1;
    for y in 0..rows {
        let mut buffer: String = (0..cols)
            .map(|x| {
                if dots.contains(&Point { x, y }) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();
        buffer.push('\n');
        writer
            .write_all(buffer.as_bytes())
            .context("error while writing grid info")?;
    }
    Ok(())
}
