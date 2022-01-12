//! Day 20: Trench Map, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/20>
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

use anyhow::{bail, ensure, Context};
use clap::Parser;

use aoc2021::argparser::Cli;
use aoc2021::collect_array::CollectArray;
use aoc2021::vecmat::CardinalVector;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input {
        enhancer_lookup,
        light_pixels,
    } = Input::from_buffer(input_reader).expect("cannot parse input");

    eprintln!("{:?}", enhancer_lookup);
    eprintln!("{:?}", light_pixels);

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
    enhancer_lookup: [u8; 512],
    light_pixels: HashSet<Point2D>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let enhancer_lookup = {
            let line = lines.next().context("expected first line")??;
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Ok(0),
                    '#' => Ok(1),
                    _ => bail!("invalid char: '{}'", c.escape_default()),
                })
                .try_collect_exact()?
        };

        let break_line = lines.next().context("expected empty second line")??;
        ensure!(break_line.trim().is_empty(), "expected empty second line");

        let mut light_pixels = HashSet::new();
        for (i, line) in lines.enumerate() {
            for (j, c) in line?.trim().chars().enumerate() {
                if c == '#' {
                    light_pixels.insert(Point2D::new([i as i64, j as i64]));
                }
            }
        }

        Ok(Input {
            enhancer_lookup,
            light_pixels,
        })
    }
}

type Point2D = CardinalVector<i64, 2>;
