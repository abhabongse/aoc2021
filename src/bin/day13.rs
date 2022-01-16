//! Day 13: Transparent Origami, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/13>
use std::io;
use std::io::{BufRead, BufReader, Write};

use anyhow::{bail, ensure, Context};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::Cli;
use aoc2021::collect_array::CollectArray;
use aoc2021::hashing::HashSet;
use aoc2021::parsing::QuickParse;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { dots, fold_instrs } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: First fold only
    let p1_dot_count = {
        let dots: HashSet<Point> = dots
            .iter()
            .map(|dot| fold_instrs[0].fold_point(*dot))
            .collect();
        dots.len()
    };
    println!("Part 1 answer: {}", p1_dot_count);

    // Part 2: Fold and print result
    let dots: HashSet<Point> = fold_instrs
        .iter()
        .fold(dots.into_iter().collect(), |dots, instr| {
            dots.into_iter().map(|dot| instr.fold_point(dot)).collect()
        });
    let mut debug_writer = io::LineWriter::new(io::stdout());
    println!("Part 2 answer: (see below)");
    write_dots(&mut debug_writer, &dots).expect("error while printing dots to stderr");
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Collection of initial dots on paper
    dots: Vec<Point>,
    /// Sequence of folding instructions
    fold_instrs: Vec<FoldInstr>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut dots = Vec::new();
        let mut fold_instrs = Vec::new();

        let mut lines = reader.lines();
        for line in lines.by_ref() {
            let line = line.context("cannot read a line of string")?;
            if line.trim().is_empty() {
                break;
            }
            let [x, y] = line.split(',').collect_exact()?;
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
                .with_context(|| format!("invalid folding instruction: {}", line))?;
            let c = captures[2].quickparse()?;
            let instr = match &captures[1] {
                "x" => FoldInstr::XEquals(c),
                "y" => FoldInstr::YEquals(c),
                axis => bail!("unrecognized axis: {}", axis),
            };
            fold_instrs.push(instr);
        }
        Ok(Input { dots, fold_instrs })
    }
}

/// Point in two-dimensional space
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    /// x-coordinate
    x: u64,
    /// y-coordinate
    y: u64,
}

/// Fold instructions
#[derive(Debug, Clone, Copy)]
enum FoldInstr {
    /// Folding along the line X = c
    XEquals(u64),
    /// Folding along the line Y = c
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

/// Prints the dots as the debugging mechanisms
fn write_dots(writer: &mut impl Write, dots: &HashSet<Point>) -> anyhow::Result<()> {
    ensure!(!dots.is_empty(), "empty dots specified");
    let nrows = *dots.iter().map(|Point { x: _, y }| y).max().unwrap() + 1;
    let ncols = *dots.iter().map(|Point { x, y: _ }| x).max().unwrap() + 1;
    for y in 0..nrows {
        let mut buffer: String = (0..ncols)
            .map(|x| {
                if dots.contains(&Point { x, y }) {
                    "🟨"
                } else {
                    "⬛️" // this emoji contains two codepoints
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
