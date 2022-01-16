//! Day 15: Chiton, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/15>
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};

use anyhow::Context;
use clap::Parser;
use nalgebra::{DMatrix, RowDVector};

use aoc2021::argparser::Cli;
use aoc2021::grid::{GridPoint, OrthAdjacent};
use aoc2021::hashing::HashMap;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { risk_levels } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: For input grid
    let p1_answer = {
        let (nrows, ncols) = risk_levels.shape();
        let grid_proxy = GridProxy {
            shape: (nrows, ncols),
            proxy_map: |pos: GridPoint<usize>| -> i64 { risk_levels[pos] },
        };
        shortest_path(&grid_proxy, (0, 0), (nrows - 1, ncols - 1))
    };
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: For 5×5 extended input grid
    let p2_answer = {
        let (nrows, ncols) = risk_levels.shape();
        let grid_proxy = GridProxy {
            shape: (5 * nrows, 5 * ncols),
            proxy_map: Box::new(|(i, j): GridPoint<usize>| -> i64 {
                let item = risk_levels[(i % nrows, j % ncols)] + (i / nrows + j / ncols) as i64;
                match item % 9 {
                    0 => 9,
                    d => d,
                }
            }),
        };
        shortest_path(&grid_proxy, (0, 0), (5 * nrows - 1, 5 * ncols - 1))
    };
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Risk levels of each position in a grid
    risk_levels: DMatrix<i64>,
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
        let risk_levels = DMatrix::from_rows(elements.as_slice());
        Ok(Input { risk_levels })
    }
}

/// Computes the length of the shortest path from `start` to `end` within the grid.
/// Such length consists of the weight sum of all nodes in the part except the start.
fn shortest_path<F>(grid: &GridProxy<i64, F>, start: GridPoint<usize>, end: GridPoint<usize>) -> i64
where
    F: Fn(GridPoint<usize>) -> i64,
{
    let mut pq = BinaryHeap::from([State {
        pos: start,
        cost: 0,
    }]);
    let mut dists: HashMap<GridPoint<usize>, i64> = HashMap::from_iter([(start, 0)]);
    while let Some(State { cost, pos }) = pq.pop() {
        if pos == end {
            return cost;
        }
        if cost > dists.get(&pos).copied().unwrap_or(i64::MAX) {
            continue;
        }
        for other_pos in OrthAdjacent::new(pos).within_shape(grid.shape) {
            let next = State {
                cost: cost + (grid.proxy_map)(other_pos),
                pos: other_pos,
            };
            if next.cost < dists.get(&next.pos).copied().unwrap_or(i64::MAX) {
                pq.push(next);
                dists.insert(next.pos, next.cost);
            }
        }
    }
    unreachable!()
}

/// Represents the state of each node in priority queue for Dijkstra's algorithm
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    pos: GridPoint<usize>,
    cost: i64,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

/// Proxy for grid type with item looking being computed on-the-fly
struct GridProxy<T, F>
where
    F: Fn(GridPoint<usize>) -> T,
{
    shape: GridPoint<usize>,
    proxy_map: F,
}
