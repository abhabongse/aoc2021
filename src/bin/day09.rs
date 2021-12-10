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

/// An iterator which generates up to four positions orthogonally adjacent to the given `pos`
/// bounded between `(0..rows, 0..cols)` where `(rows, cols) = clip_size`.
fn orthogonal_neighbors(
    pos: (usize, usize),
    clip_size: (usize, usize),
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    let iter = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(move |(di, dj)| (pos.0 as i64 + di, pos.1 as i64 + dj))
        .filter(move |(ni, nj)| {
            (0..clip_size.0 as i64).contains(ni) && (0..clip_size.1 as i64).contains(nj)
        })
        .map(move |(ni, nj)| (ni as usize, nj as usize));
    Box::new(iter)
}
