//! Day 4: Giant Squid, Advent of Code 2021
//! <https://adventofcode.com/2021/day/4>
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::iter::Sum;
use std::ops::Not;
use std::str::FromStr;

use anyhow::anyhow;
use itertools::{iproduct, Itertools};
use num::PrimInt;

use aoc2021::argparser;
use aoc2021::collections::TryCollectArray;
use aoc2021::quickparse::QuickParse;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let Input { boards, lots } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Plays each bingo board with the pre-determined list of lots until reaching the winning state
    // and records the final result consisting of the score and the number of rounds played.
    let play_results: Vec<_> = boards
        .iter()
        .map(|board| board.play_with_lots(lots.as_slice()))
        .collect();

    // Part 1: first bingo board to win
    let p1_answer = play_results
        .iter()
        .min_by_key(|result| result.rounds_played)
        .expect("no bingo boards read from the input")
        .score
        .expect("unfinished board; score unavailable");
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: last bingo board to win
    let p2_answer = play_results
        .iter()
        .max_by_key(|result| result.rounds_played)
        .expect("no bingo boards read from the input")
        .score
        .expect("unfinished board; score unavailable");
    println!("Part 2 answer: {}", p2_answer);
}

/// Represents input data for the problem
#[derive(Debug, Clone)]
struct Input {
    lots: Vec<i64>,
    boards: Vec<Board<i64, 5, 5>>,
}

impl Input {
    /// Reads the input data from a buffer reader.
    ///
    /// # Implementation Note
    /// An earlier version of this method short-circuits the error at the earliest convenience.
    /// However, this behavior was removed due to growing code complexity from such implementation.
    /// Perhaps, some lesson has been learned the hard way.
    /// TODO: Learn how to parse input from buffer stream with proper short-circuit error handling
    fn from_buffer<R: BufRead>(reader: R) -> anyhow::Result<Self> {
        let lines: Vec<_> = reader.lines().collect::<Result<_, io::Error>>()?;
        let mut batches: VecDeque<_> = lines.into_iter().batching(collect_batch).collect();

        let lots = batches
            .pop_front()
            .ok_or(anyhow!("missing lots data"))?
            .iter()
            .flat_map(|line| line.split(','))
            .map(|token| token.trim().quickparse())
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
#[derive(Debug, Clone)]
struct Board<T, const R: usize, const C: usize>
where
    T: PrimInt,
{
    /// Grid numbers of the bingo board.
    numbers: [[T; C]; R],
    /// Auxiliary data mapping from a number to the indexing position on the bingo board.
    mapper: HashMap<T, (usize, usize)>,
}

impl<T, const R: usize, const C: usize> Board<T, R, C>
where
    T: PrimInt,
{
    /// Constructs a bingo board from 2-d array grid of numbers.
    fn new(numbers: [[T; C]; R]) -> Self
    where
        T: Hash,
    {
        let mapper: HashMap<T, (usize, usize)> = numbers
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(j, value)| (value, (i, j)))
            })
            .collect();
        Board { numbers, mapper }
    }

    /// Constructs a bingo board from a vector of strings
    /// where each string represents a bingo row containing numbers separated by whitespaces.
    fn from_lines(lines: Vec<String>) -> anyhow::Result<Self>
    where
        T: Hash + FromStr,
    {
        let numbers: Vec<Vec<T>> = lines
            .into_iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|token| token.trim().quickparse())
                    .collect::<anyhow::Result<_>>()
            })
            .collect::<anyhow::Result<_>>()?;
        Board::try_from(numbers)
    }

    /// Plays the bingo board from the beginning with the given sequences of lots,
    /// and returns the final score and the number of rounds played.
    fn play_with_lots(&self, lots: &[T]) -> PlayResult<T>
    where
        T: Hash + Sum,
    {
        let mut checker = self.new_checker();
        for (i, lot) in lots.iter().copied().enumerate() {
            let score = checker.mark(lot);
            if score.is_some() {
                return PlayResult {
                    score,
                    rounds_played: i,
                };
            }
        }
        PlayResult {
            score: None,
            rounds_played: lots.len(),
        }
    }

    /// Constructs a bingo board checker from the current board.
    fn new_checker(&self) -> BoardChecker<T, R, C> {
        BoardChecker {
            board: self,
            marks: [[false; C]; R],
            score: None,
        }
    }
}

impl<T, const R: usize, const C: usize> TryFrom<Vec<Vec<T>>> for Board<T, R, C>
where
    T: PrimInt + Hash,
{
    type Error = anyhow::Error;

    fn try_from(numbers: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Ok(Board::new(
            numbers
                .into_iter()
                .map(|row| row.into_iter().try_collect_exact_array::<_, C>())
                .collect::<anyhow::Result<Vec<[_; C]>>>()?
                .into_iter()
                .try_collect_exact_array::<_, R>()?,
        ))
    }
}

/// A bingo board checker optimizes for bingo checking.
#[derive(Debug, Clone)]
struct BoardChecker<'a, T, const R: usize, const C: usize>
where
    T: PrimInt,
{
    /// The original bingo board.
    board: &'a Board<T, R, C>,
    /// Records markings of which cells on the board have been called.
    marks: [[bool; C]; R],
    /// Tracks the final score. None if it has not reached the winning state just yet.
    score: Option<T>,
}

impl<T, const R: usize, const C: usize> BoardChecker<'_, T, R, C>
where
    T: PrimInt,
{
    /// Marks a called lot on the bingo board and finalizes the score
    /// if the board has achieved the winning state.
    /// Subsequent marks after initial winning mark may result in incorrect score.
    fn mark(&mut self, call: T) -> Option<T>
    where
        T: Hash + Sum,
    {
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
    fn sum_unmarked(&self) -> T
    where
        T: Sum,
    {
        iproduct!(0..R, 0..C)
            .filter_map(|(i, j)| (!self.marks[i][j]).then(|| self.board.numbers[i][j]))
            .sum()
    }
}

/// Data representing the result from playing a bingo game.
#[derive(Debug, Clone, Copy)]
struct PlayResult<T> {
    /// Final Score of the bingo board. None if the board has reached the winning state.
    score: Option<T>,
    /// Counts the number of called lots until the board reaches the winning state.
    /// If the winning state is not reached, it still stores the total number of lots called.
    rounds_played: usize,
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
