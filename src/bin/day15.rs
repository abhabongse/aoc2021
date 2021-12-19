//! Day 15: Chiton, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/15>
use anyhow::Context;
use nalgebra::{DMatrix, Dim, Matrix, RowDVector, StorageMut};
use std::io::BufRead;

use aoc2021::argparser;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { risk_levels } = Input::from_buffer(input_reader).expect("cannot parse input");

    eprintln!("{:?}", risk_levels);

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
    risk_levels: DMatrix<i64>,
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
                    .with_context(|| format!("invalid character in decimal string: {}", c))?
                    as i64;
                row_elements.push(d)
            }
            elements.push(RowDVector::from_vec(row_elements));
        }
        let risk_levels = DMatrix::from_rows(elements.as_slice());
        Ok(Input { risk_levels })
    }
}

fn shortest_path<R, C, S>(grid: &Matrix<i64, R, C, S>) -> Option<usize>
where
    R: Dim,
    C: Dim,
    S: StorageMut<i64, R, C>,
{
}
