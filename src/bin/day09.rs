//! Day 9: Smoke Basin, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/9>
use std::cmp::Reverse;
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader};

use anyhow::Context;
use clap::Parser;
use itertools::Itertools;
use nalgebra::{DMatrix, Dim, Matrix, RawStorage, RowDVector};

use aoc2021::argparser::Cli;
use aoc2021::grid::{GridIndices, OrthAdjacent};

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { heightmap } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Find all low points in the heightmap
    let low_points: Vec<_> = GridIndices::row_major(heightmap.shape())
        .filter(|&pos| {
            OrthAdjacent::new(pos)
                .within_shape(heightmap.shape())
                .all(|other_pos| heightmap[pos] < heightmap[other_pos])
        })
        .collect();

    // Part 1: Sum or risk levels of the seafloor heightmap
    let p1_answer: i64 = low_points.iter().map(|&pos| heightmap[pos] + 1).sum();
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Find three largest basins
    let p2_answer: usize = {
        let basin_sizes = low_points.iter().map(|&pos| basin_size(pos, &heightmap));
        let top_basin_sizes = basin_sizes.map(Reverse).k_smallest(3).map(|s| s.0);
        top_basin_sizes.into_iter().product()
    };
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Heightmap of the seafloor in two dimensions
    heightmap: DMatrix<i64>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut elements = Vec::new();
        for line in reader.lines() {
            let mut row_elements = Vec::new();
            for c in line?.trim().chars() {
                let d = c.to_digit(10).with_context(|| {
                    format!(
                        "invalid character in decimal string: '{}'",
                        c.escape_default()
                    )
                })? as i64;
                row_elements.push(d)
            }
            elements.push(RowDVector::from_vec(row_elements));
        }
        let heightmap = DMatrix::from_rows(elements.as_slice());
        Ok(Input { heightmap })
    }
}

/// Uses breadth-first search to find the basin
/// whose low point is the same as given in the function parameter.
fn basin_size<R, C, S>(low_point: (usize, usize), heightmap: &Matrix<i64, R, C, S>) -> usize
where
    R: Dim,
    C: Dim,
    S: RawStorage<i64, R, C>,
{
    let shape = heightmap.shape();
    let mut queue = VecDeque::from([low_point]);
    let mut visited = HashSet::from([low_point]);
    while let Some(pos) = queue.pop_front() {
        for other_pos in OrthAdjacent::new(pos).within_shape(shape) {
            if heightmap[other_pos] < 9 && !visited.contains(&other_pos) {
                queue.push_back(other_pos);
                visited.insert(other_pos);
            }
        }
    }
    visited.len()
}
