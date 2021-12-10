//! Day 9: Smoke Basin, Advent of Code 2021
//! https://adventofcode.com/2021/day/9
use std::io::BufRead;

use anyhow::{bail, Context};
use nalgebra::{DMatrix, RowDVector};

use aoc2021::argparser;
use aoc2021::nalgebra::MatrixExt;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let heightmap = parse_input(input_reader).expect("cannot parse input");

    // Part 1: Sum or risk levels
    let p1_answer = 0;
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses two-dimensional heightmap of the seafloor.
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<DMatrix<u8>> {
    let elements: Vec<_> = reader
        .lines()
        .map(|line| {
            let row_elements: Vec<_> = line
                .context("cannot read a line of string from input buffer")?
                .trim()
                .bytes()
                .map(|c| match c {
                    b'0'..=b'9' => Ok(c - b'0'),
                    _ => bail!("invalid character in decimal string: {}", c),
                })
                .collect::<anyhow::Result<_>>()?;
            Ok(RowDVector::from_vec(row_elements))
        })
        .collect::<anyhow::Result<_>>()?;
    Ok(DMatrix::from_rows(elements.as_slice()))
}
