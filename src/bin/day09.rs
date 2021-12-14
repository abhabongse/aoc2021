//! Day 9: Smoke Basin, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/9>
use std::cmp::Reverse;
use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

use anyhow::anyhow;
use itertools::{iproduct, Itertools};
// TODO: Stop using nalgebra, use homegrown grid
use nalgebra::{DMatrix, RowDVector};

use aoc2021::argparser;
use aoc2021::grid::orthogonal_neighbors;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { heightmap } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Find all low points in the heightmap
    let (rows, cols) = heightmap.shape();
    let low_points: Vec<_> = iproduct!(0..rows, 0..cols)
        .filter(|pos| {
            orthogonal_neighbors(*pos, (rows, cols))
                .into_iter()
                .all(|other_pos| heightmap[*pos] < heightmap[other_pos])
        })
        .collect();

    // Part 1: Sum or risk levels of the seafloor heightmap
    let p1_answer: i64 = low_points.iter().map(|pos| heightmap[*pos] + 1).sum();
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Find three largest basins
    let p2_answer: usize = {
        let basin_sizes = low_points.iter().map(|pos| basin_size(*pos, &heightmap));
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
                let d = c
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("invalid character in decimal string: {}", c))?
                    as i64;
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
