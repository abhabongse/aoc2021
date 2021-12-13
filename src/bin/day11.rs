//! Day 11: Dumbo Octopus, Advent of Code 2021
//! <https://adventofcode.com/2021/day/11>
use std::collections::{HashSet, VecDeque};
use std::io;
use std::io::{BufRead, Write};
use std::ops::ControlFlow;

use anyhow::{anyhow, ensure, Context};

use aoc2021::argparser;
use aoc2021::grid::{king_step_neighbors, FixedGrid};

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let grid: FixedGrid<_, 10, 10> = parse_input(input_reader).expect("cannot parse input");

    // Check the input grid
    let mut debug_writer = io::LineWriter::new(io::stderr());
    write_grid(&mut debug_writer, &grid).expect("error while printing a grid to stderr");

    // Part 1: number of flashes after 100 steps
    let p1_answer: usize = {
        let mut grid = grid.clone();
        (0..100).map(|_| update_grid(&mut grid)).sum()
    };
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: number of steps to get first simultaneous flashes
    let p2_answer: usize = {
        let mut grid = grid;
        let result = (1..).try_for_each(|i| {
            update_grid(&mut grid);
            if grid_just_all_flashed(&grid) {
                ControlFlow::Break(i)
            } else {
                ControlFlow::Continue(())
            }
        });
        match result {
            ControlFlow::Continue(_) => unreachable!(),
            ControlFlow::Break(attempts) => attempts,
        }
    };
    println!("Part 2 answer: {}", p2_answer);
}

/// Parses the energy level of octopuses in 10 by 10 grid.
fn parse_input<BR: BufRead, const R: usize, const C: usize>(
    reader: BR,
) -> anyhow::Result<FixedGrid<u8, R, C>> {
    let grid: Vec<_> = reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            ensure!(i < R, "too many lines read");
            let line = line.context("cannot read a line of string")?;
            let row: Vec<_> = line
                .trim()
                .chars()
                .map(|c| {
                    Ok(c.to_digit(10)
                        .ok_or_else(|| anyhow!("unrecognized digit: {}", c))?
                        as u8)
                })
                .collect::<anyhow::Result<_>>()?;
            Ok(row)
        })
        .collect::<anyhow::Result<_>>()?;
    FixedGrid::try_from(grid)
}

/// Updates the state of octopus grid in-place, and returns the number of flashed octopuses.
fn update_grid<const R: usize, const C: usize>(grid: &mut FixedGrid<u8, R, C>) -> usize {
    let mut queue = VecDeque::new();
    let mut marked = HashSet::new();

    // Step 1: Increment energy level of each grid cell by one
    for pos in grid.indices_by_row() {
        grid[pos] += 1;
        if grid[pos] >= 10 {
            queue.push_back(pos);
            marked.insert(pos);
        }
    }

    // Step 2: Resolve the triggering chain of flashes
    while let Some(pos) = queue.pop_front() {
        for other_pos in king_step_neighbors(pos, (R, C)) {
            grid[other_pos] += 1;
            if grid[other_pos] >= 10 && !marked.contains(&other_pos) {
                queue.push_back(other_pos);
                marked.insert(other_pos);
            }
        }
    }

    // Step 3: Clear the energy level of flashed grid cells
    for pos in grid.indices_by_row() {
        if grid[pos] >= 10 {
            grid[pos] = 0;
        }
    }

    marked.len()
}

/// Checks that all octopuses in the grid has just simultaneously flashed
/// (i.e. they have all just reset to zero).
fn grid_just_all_flashed<const R: usize, const C: usize>(grid: &FixedGrid<u8, R, C>) -> bool {
    grid.indices_by_row().all(|pos| grid[pos] == 0)
}

/// Printing the grid as the debugging method.
/// - TODO: Learn proper logging best practices.
fn write_grid<W: Write, const R: usize, const C: usize>(
    writer: &mut W,
    grid: &FixedGrid<u8, R, C>,
) -> anyhow::Result<()> {
    for i in 0..R {
        let mut buffer: String = (0..C)
            .map(|j| char::from_digit(grid[(i, j)] as u32, 10).unwrap())
            .collect();
        buffer.push('\n');
        writer
            .write_all(buffer.as_bytes())
            .expect("error while writing grid info");
    }
    Ok(())
}
