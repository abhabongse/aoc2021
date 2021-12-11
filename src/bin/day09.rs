//! Day 9: Smoke Basin, Advent of Code 2021
//! <https://adventofcode.com/2021/day/9>
use std::cmp::Reverse;
use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

use anyhow::{bail, Context};
use itertools::{iproduct, Itertools};
use nalgebra::{DMatrix, RowDVector};

use aoc2021::argparser;
use aoc2021::grid::orthogonal_neighbors;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let heightmap = parse_input(input_reader).expect("cannot parse input");

    let (rows, cols) = heightmap.shape();
    let low_points: Vec<_> = iproduct!(0..rows, 0..cols)
        .filter(|pos| {
            orthogonal_neighbors(*pos, (rows, cols))
                .into_iter()
                .all(|other_pos| heightmap[*pos] < heightmap[other_pos])
        })
        .collect();

    // Part 1: Sum or risk levels of the seafloor heightmap.
    let p1_answer: i64 = low_points.iter().map(|pos| heightmap[*pos] + 1).sum();
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Find three largest basins.
    let basin_sizes = low_points.iter().map(|pos| basin_size(*pos, &heightmap));
    let top_basin_sizes = basin_sizes.map(Reverse).k_smallest(3).map(|s| s.0);
    let p2_answer: usize = top_basin_sizes.into_iter().product();
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses two-dimensional heightmap of the seafloor.
/// TODO: Adopt https://doc.rust-lang.org/std/primitive.char.html#method.to_digit
/// TODO: Replaces [`nalgebra::Matrix`] with homegrown grid
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

/// Uses breadth-first search to find the basin
/// whose low point is the same as given in the function parameter.
fn basin_size(low_point: (usize, usize), heightmap: &DMatrix<i64>) -> usize {
    let shape = heightmap.shape();
    let mut queue = VecDeque::from([low_point]);
    let mut visited = HashSet::from([low_point]);
    while let Some(pos) = queue.pop_front() {
        orthogonal_neighbors(pos, shape)
            .into_iter()
            .for_each(|other_pos| {
                if heightmap[other_pos] < 9 && !visited.contains(&other_pos) {
                    queue.push_back(other_pos);
                    visited.insert(other_pos);
                }
            });
    }
    visited.len()
}
