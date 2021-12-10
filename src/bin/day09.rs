//! Day 9: Smoke Basin, Advent of Code 2021
//! https://adventofcode.com/2021/day/9
use std::io::BufRead;

use anyhow::{bail, Context};
use itertools::iproduct;
use nalgebra::{DMatrix, RowDVector};

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let heightmap = parse_input(input_reader).expect("cannot parse input");

    // Part 1: Sum or risk levels of the seafloor heightmap
    let (rows, cols) = heightmap.shape();
    let p1_answer: i64 = iproduct!(0..rows, 0..cols)
        .filter_map(|pos| {
            orthogonal_neighbors(pos, (rows, cols))
                .into_iter()
                .all(|other_pos| heightmap[pos] < heightmap[other_pos])
                .then(|| heightmap[pos] + 1)
        })
        .sum();
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses two-dimensional heightmap of the seafloor.
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<DMatrix<i64>> {
    let elements: Vec<_> = reader
        .lines()
        .map(|line| {
            let row_elements: Vec<_> = line
                .context("cannot read a line of string from input buffer")?
                .trim()
                .bytes()
                .map(|c| match c {
                    b'0'..=b'9' => Ok(c as i64 - b'0' as i64),
                    _ => bail!("invalid character in decimal string: {}", c),
                })
                .collect::<anyhow::Result<_>>()?;
            Ok(RowDVector::from_vec(row_elements))
        })
        .collect::<anyhow::Result<_>>()?;
    Ok(DMatrix::from_rows(elements.as_slice()))
}

/// Obtains a list of up to four positions which are orthogonally adjacent to the given `pos`
/// and are bounded within `(0..rows, 0..cols)` where `(rows, cols) == clip_size`.
fn orthogonal_neighbors(pos: (usize, usize), clip_size: (usize, usize)) -> Vec<(usize, usize)> {
    fn clipped_add(a: usize, b: i64, size: usize) -> Option<usize> {
        let total = (a as i64) + b;
        (0..size as i64).contains(&total).then(|| total as usize)
    }
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(di, dj)| {
            Some((
                clipped_add(pos.0, di, clip_size.0)?,
                clipped_add(pos.1, dj, clip_size.1)?,
            ))
        })
        .collect()
}
