//! Day 4: Giant Squid, Advent of Code 2021
//! https://adventofcode.com/2021/day/4
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::io::BufRead;
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

/// Represents input data for the problem
#[derive(Debug, Clone)]
struct Input {
    lots: Vec<usize>,
    boards: Vec<Board<usize, 5, 5>>,
}

impl Input {
    /// Constructs the input data from a buffer reader
    ///
    /// An earlier version of this method short-circuits the error at the earliest convenience.
    /// This behavior was removed due to the growing code complexity from such implementation.
    fn from_buffer<R: BufRead>(reader: R) -> anyhow::Result<Self> {
        let lines: Vec<_> = reader.lines().collect::<Result<_, io::Error>>()?;
        let mut batches: VecDeque<_> = lines.into_iter().batching(Input::collect_batch).collect();

        let lots = batches
            .pop_front()
            .ok_or(anyhow!("missing lots data"))?
            .iter()
            .map(|line| line.split(','))
            .flatten()
            .map(|token| Ok(token.trim().parse()?))
            .collect::<anyhow::Result<_>>()?;

        let boards: Vec<_> = batches
            .into_iter()
            .map(Board::from_lines)
            .collect::<anyhow::Result<_>>()?;

        Ok(Input { lots, boards })
    }

    /// Collects strings from the given iterator into a vector until a seemingly empty string is reached.
    /// If the iterator is already exhausted in the first place, then None is returned.
    /// Note that a string containing just whitespaces is considered empty, and will not be part of output.
    fn collect_batch<I: Iterator<Item = String>>(it: &mut I) -> Option<Vec<String>> {
        let mut collected: Vec<String> = Vec::new();
        for line in it {
            if line.trim().is_empty() {
                return Some(collected);
            }
            collected.push(line);
        }
        // Iterator is exhausted; check if the last few lines exists
        match collected.as_slice() {
            [] => None,
            _ => Some(collected),
        }
    }
}

/// A bingo board with flexible sizes and element type
///  - T is the type of board cell element
///  - R is the number of rows
///  - C is the number of columns
#[derive(Debug, Clone)]
struct Board<T, const R: usize, const C: usize>([[T; C]; R]);

impl<T, const R: usize, const C: usize> Board<T, R, C> {
    /// Constructs a bingo board from a vector of strings
    /// where each string represents a bingo row containing numbers separated by whitespaces.
    fn from_lines(lines: Vec<String>) -> anyhow::Result<Self>
    where
        T: FromStr,
        <T as FromStr>::Err: 'static + Error + Sync + Send,
    {
        let board_numbers: [[T; C]; R] = lines
            .into_iter()
            .map(|line| {
                let row_numbers: [T; C] = line
                    .split_ascii_whitespace()
                    .map(|token| Ok(token.trim().parse()?))
                    .collect::<anyhow::Result<Vec<T>>>()?
                    .try_into()
                    .map_err(|v: Vec<_>| {
                        anyhow!("found a board with {} columns instead of {}", v.len(), C)
                    })?;
                Ok(row_numbers)
            })
            .collect::<anyhow::Result<Vec<[T; C]>>>()?
            .try_into()
            .map_err(|v: Vec<_>| anyhow!("found a board with {} rows instead of {}", v.len(), R))?;
        Ok(Board(board_numbers))
    }
}
