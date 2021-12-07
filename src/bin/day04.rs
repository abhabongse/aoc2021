//! Day 4: Giant Squid, Advent of Code 2021
//! https://adventofcode.com/2021/day/4
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::io;
use std::io::BufRead;
use std::ops::Not;

use anyhow::anyhow;
use itertools::{iproduct, Itertools};

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let input = Input::from_buffer(input_reader).expect("cannot parse input");

    // Plays each bingo board with the pre-determined list of lots until reaching the winning state
    // and records the final result consisting of the score and the number of rounds played.
    let results: Vec<_> = input
        .boards
        .iter()
        .map(|board| {
            let mut checker = board.new_checker();
            for (i, lot) in input.lots.iter().copied().enumerate() {
                let score = checker.mark(lot);
                if score.is_some() {
                    return PlayResult { score, rounds: i };
                }
            }
            PlayResult {
                score: None,
                rounds: input.lots.len(),
            }
        })
        .collect();

    let p1_answer = results
        .iter()
        .min_by_key(|result| result.rounds)
        .expect("no bingo boards read from the input")
        .score
        .expect("unfinished board; score unavailable");
    println!("Part 1 answer: {}", p1_answer);

    let p2_answer = results
        .iter()
        .max_by_key(|result| result.rounds)
        .expect("no bingo boards read from the input")
        .score
        .expect("unfinished board; score unavailable");
    println!("Part 2 answer: {}", p2_answer);
}

/// Represents input data for the problem
#[derive(Debug, Clone)]
struct Input {
    lots: Vec<isize>,
    boards: Vec<Board<5, 5>>,
}

impl Input {
    /// Reads the input data from a buffer reader.
    ///
    /// # Implementation Note
    /// An earlier version of this method short-circuits the error at the earliest convenience.
    /// However, this behavior was removed due to growing code complexity from such implementation.
    /// Perhaps, some lesson has been learned the hard way.
    fn from_buffer<R: BufRead>(reader: R) -> anyhow::Result<Self> {
        let lines: Vec<_> = reader.lines().collect::<Result<_, io::Error>>()?;
        let mut batches: VecDeque<_> = lines.into_iter().batching(collect_batch).collect();

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
}

/// A bingo board with flexible sizes and element type,
/// where `R` and `C` is the number of rows and columns of the board, respectively.
/// TODO: make this board generic by declaring number implementing traits [`num::Integer`]
///
/// [`num::Integer`]: https://docs.rs/num/latest/num/trait.Integer.html
#[derive(Debug, Clone)]
struct Board<const R: usize, const C: usize> {
    /// Grid numbers of the bingo board.
    numbers: [[isize; C]; R],
    /// Auxiliary data mapping from a number to the indexing position on the bingo board.
    mapper: HashMap<isize, (usize, usize)>,
}

impl<const R: usize, const C: usize> Board<R, C> {
    /// Constructs a bingo board from 2-d array grid of numbers.
    fn new(numbers: [[isize; C]; R]) -> Self {
        let mapper: HashMap<isize, (usize, usize)> = numbers
            .into_iter()
            .enumerate()
            .map(|(i, r)| r.into_iter().enumerate().map(move |(j, v)| (v, (i, j))))
            .flatten()
            .collect();
        Board { numbers, mapper }
    }

    /// Constructs a bingo board checker from the current board.
    fn new_checker(&self) -> BoardChecker<R, C> {
        BoardChecker {
            board: self,
            marks: [[false; C]; R],
            score: None,
        }
    }

    /// Constructs a bingo board from a vector of strings
    /// where each string represents a bingo row containing numbers separated by whitespaces.
    fn from_lines(lines: Vec<String>) -> anyhow::Result<Self> {
        let numbers: Vec<Vec<isize>> = lines
            .into_iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|token| Ok(token.trim().parse::<_>()?))
                    .collect::<anyhow::Result<_>>()
            })
            .collect::<anyhow::Result<_>>()?;
        Board::try_from(numbers)
    }
}

impl<const R: usize, const C: usize> TryFrom<Vec<Vec<isize>>> for Board<R, C> {
    type Error = anyhow::Error;

    fn try_from(numbers: Vec<Vec<isize>>) -> Result<Self, Self::Error> {
        let board: [[_; C]; R] = numbers
            .into_iter()
            .map(|row| {
                let row: [_; C] = row.try_into().map_err(|v: Vec<_>| {
                    anyhow!("found a board with {} columns instead of {}", v.len(), C)
                })?;
                Ok(row)
            })
            .collect::<anyhow::Result<Vec<[_; C]>>>()?
            .try_into()
            .map_err(|v: Vec<_>| anyhow!("found a board with {} rows instead of {}", v.len(), R))?;
        Ok(Board::new(board))
    }
}

/// A bingo board checker optimizes for bingo checking.
#[derive(Debug, Clone)]
struct BoardChecker<'a, const R: usize, const C: usize> {
    /// The original bingo board.
    board: &'a Board<R, C>,
    /// Records markings of which cells on the board have been called.
    marks: [[bool; C]; R],
    /// Tracks the final score. None if it has not reached the winning state just yet.
    score: Option<isize>,
}

impl<const R: usize, const C: usize> BoardChecker<'_, R, C> {
    /// Marks a called lot on the bingo board and finalizes the score
    /// if the board has achieved the winning state.
    /// Subsequent marks after initial winning mark may result in incorrect score.
    fn mark(&mut self, call: isize) -> Option<isize> {
        if self.score.is_none() {
            if let Some((i, j)) = self.board.mapper.get(&call).copied() {
                self.marks[i][j] = true;
                if self.check_row_winning(i) || self.check_col_winning(j) {
                    self.score = Some(call * self.sum_unmarked())
                }
            }
        }
        self.score
    }

    /// Checks whether a given row has achieved the winning state
    fn check_row_winning(&self, row: usize) -> bool {
        (0..C).map(|j| self.marks[row][j]).all(|x| x)
    }

    /// Checks whether a given column has achieved the winning state
    fn check_col_winning(&self, col: usize) -> bool {
        (0..R).map(|i| self.marks[i][col]).all(|x| x)
    }

    /// Sum of unmarked numbers on the bingo board
    fn sum_unmarked(&self) -> isize {
        iproduct!(0..R, 0..C)
            .map(|(i, j)| self.board.numbers[i][j] * (!self.marks[i][j]) as isize)
            .sum()
    }
}

/// Data representing the result from playing a bingo game.
#[derive(Debug, Clone, Copy)]
struct PlayResult {
    /// Final Score of the bingo board. None if the board has reached the winning state.
    score: Option<isize>,
    /// Counts the number of called lots until the board reaches the winning state.
    /// If the winning state is not reached, it still stores the total number of lots called.
    rounds: usize,
}

/// Collects strings from the iterator into a vector until a seemingly empty string
/// (including those containing just whitespaces) has been found.
/// Empty strings will not be included as part of the returned vector.
/// If the iterator has already been exhausted in the first place, None is returned.
fn collect_batch<I>(it: &mut I) -> Option<Vec<String>>
where
    I: Iterator<Item = String>,
{
    let mut collected: Vec<String> = Vec::new();
    for line in it {
        if line.trim().is_empty() {
            return Some(collected);
        }
        collected.push(line);
    }
    // Checks for the remaining last few lines when iterator has been exhausted
    collected.is_empty().not().then(|| collected)
}
