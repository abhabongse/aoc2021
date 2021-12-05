//! Day 4: Giant Squid, Advent of Code 2021
//! https://adventofcode.com/2021/day/4
use std::error::Error;
use std::fmt::Debug;
use std::io::BufRead;
use std::ops::Not;
use std::str::FromStr;

use anyhow::anyhow;
use itertools::Itertools;

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let input = Input::from_buffer(input_reader).expect("cannot parse input");

    println!("{:?}", input.lots);
    println!("{:?}", input.boards);

    let p1_answer = 0;
    println!("Part 1 answer: {}", p1_answer);

    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Represents input data read from source
#[derive(Debug, Clone)]
struct Input {
    lots: Vec<usize>,
    boards: Vec<Board<usize, 5, 5>>,
}

impl Input {
    /// Constructs the input data from a buffer reader
    fn from_buffer<R: BufRead>(reader: R) -> anyhow::Result<Self> {
        // The result is an iterator which produces multiple batches, each of which separated by an empty line.
        // Each element produced by the iterator is a vector of lines belong to the same batch.
        let mut batches = reader.lines().batching(|it| {
            let result: anyhow::Result<Vec<_>> = it
                .take_while(|line| match line {
                    Ok(line) => line.trim().is_empty().not(),
                    Err(_) => false,
                })
                .map(|line| Ok(line?))
                .collect();
            match result {
                Ok(collected) if collected.is_empty() => None,
                _ => Some(result),
            }
        });

        // Parse lots from the first batch
        let lots: Vec<_> = batches
            .next()
            .ok_or(anyhow!("cannot read lots from first batch"))??
            .iter()
            .map(|line| line.split(','))
            .flatten()
            .map(|token| Ok(token.trim().parse()?))
            .collect::<anyhow::Result<_>>()?;

        // Parse boards for the remaining batches
        let boards: Vec<_> = batches
            .map(|batch| batch.and_then(Board::from_lines))
            .collect::<anyhow::Result<_>>()?;

        Ok(Input { lots, boards })
    }
}

/// Represents bingo board
#[derive(Debug, Clone)]
struct Board<T, const R: usize, const C: usize>([[T; C]; R]);

impl<T, const R: usize, const C: usize> Board<T, R, C> {
    fn from_lines(lines: Vec<String>) -> anyhow::Result<Self>
    where
        T: FromStr,
        <T as FromStr>::Err: 'static + Error + Sync + Send,
    {
        let board_numbers: Vec<[T; C]> = lines
            .into_iter()
            .map(|line| {
                let row_numbers: Vec<T> = line
                    .split_ascii_whitespace()
                    .map(|token| Ok(token.trim().parse()?))
                    .collect::<anyhow::Result<_>>()?;
                let row_numbers: [T; C] = row_numbers.try_into().map_err(|v: Vec<T>| {
                    anyhow!("found a board with {} columns instead of {}", v.len(), C)
                })?;
                Ok(row_numbers)
            })
            .collect::<anyhow::Result<_>>()?;
        let board_numbers: [[T; C]; R] = board_numbers.try_into().map_err(|v: Vec<[T; C]>| {
            anyhow!("found a board with {} rows instead of {}", v.len(), R)
        })?;
        Ok(Board(board_numbers))
    }
}
