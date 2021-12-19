//! Day 4: Giant Squid, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/4>
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::iter::Sum;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use itertools::{iproduct, Itertools};
use num::PrimInt;

use aoc2021::argparser;
use aoc2021::collect_array::CollectArray;
use aoc2021::quickparse::QuickParse;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { boards, lots } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Play each bingo board with the pre-determined sequence of lots until reaching the winning state
    // and then record the final result consisting of the score and the number of rounds played.
    if boards.is_empty() {
        panic!("there is not even a single bingo board read from input");
    }
    let play_results: Vec<_> = boards
        .iter()
        .map(|board| board.play_with_lots(lots.as_slice()))
        .collect();

    // Part 1: First bingo board to win
    let p1_first_win_score = {
        let result = play_results.iter().min_by_key(|r| r.rounds_played).unwrap();
        result.score.expect("unfinished board; score unavailable")
    };
    println!("Part 1 answer: {}", p1_first_win_score);

    // Part 2: Last bingo board to win
    let p2_last_win_score = {
        let result = play_results.iter().max_by_key(|r| r.rounds_played).unwrap();
        result.score.expect("unfinished board; score unavailable")
    };
    println!("Part 2 answer: {}", p2_last_win_score);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Sequence of drawn lots
    lots: Vec<i64>,
    /// Collection of bingo boards
    boards: Vec<Board<i64, 5, 5>>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut batches = reader.lines().batching(collect_line_batch);

        let mut lots = Vec::new();
        let batch = batches.next().context("missing lots data")??;
        for line in batch {
            for token in line.split(',') {
                lots.push(token.trim().quickparse()?);
            }
        }

        let mut boards = Vec::new();
        for batch in batches {
            boards.push(Board::from_lines(batch?)?);
        }

        Ok(Input { lots, boards })
    }
}

/// Collects strings from an iterator into a vector until a seemingly empty string
/// (which includes strings containing just whitespaces) has been found.
/// Empty strings will not be included as part of the returned vector.
/// If the iterator has already been exhausted in the first place, `None` is returned.
fn collect_line_batch<I>(it: &mut I) -> Option<anyhow::Result<Vec<String>>>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let mut buffer = Vec::new();
    for line in it {
        match line {
            Ok(s) if s.trim().is_empty() => return Some(Ok(buffer)),
            Ok(s) => buffer.push(s),
            Err(_) => {
                return Some(Err(anyhow!(
                    "error while reading a line of string from input"
                )))
            }
        }
    }
    if buffer.is_empty() {
        None
    } else {
        Some(Ok(buffer))
    }
}

/// Bingo board with compile-time constant size and flexible element type.
/// Parameters `R` and `C` are the number of rows and columns, respectively.
#[derive(Debug, Clone)]
struct Board<T, const R: usize, const C: usize>
where
    T: PrimInt,
{
    /// Number grid of the bingo board
    numbers: [[T; C]; R],
    /// Auxiliary mapping data structure from a bingo number to the indexing positio on the bingo board
    mapper: HashMap<T, (usize, usize)>,
}

impl<T, const R: usize, const C: usize> Board<T, R, C>
where
    T: PrimInt,
{
    /// Constructs a bingo board from 2-d array grid of numbers.
    /// Member `mapper` will be constructed on-the-fly.
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
        let mut board_numbers = Vec::new();
        for line in lines {
            let mut row_numbers = Vec::new();
            for token in line.split_ascii_whitespace() {
                row_numbers.push(token.trim().quickparse()?);
            }
            board_numbers.push(row_numbers);
        }
        Board::try_from(board_numbers)
    }

    /// Plays the bingo board from the beginning with the given sequence of lots,
    /// and returns the final score and the number of rounds played.
    fn play_with_lots(&self, lots: &[T]) -> PlayResult<T>
    where
        T: Hash + Sum,
    {
        let mut checker = self.spawn_checker();
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

    /// Spawns a new bingo board checker of the current board.
    fn spawn_checker(&self) -> BoardChecker<T, R, C> {
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
        let mut rows = Vec::with_capacity(R);
        for row in numbers {
            let row: [_; C] = row.into_iter().collect_exact_array()?;
            rows.push(row);
        }
        Ok(Board::new(rows.into_iter().collect_exact_array()?))
    }
}

/// Bingo board checker which optimizes for bingo checking
#[derive(Debug, Clone)]
struct BoardChecker<'a, T, const R: usize, const C: usize>
where
    T: PrimInt,
{
    /// Reference to the original bingo board
    board: &'a Board<T, R, C>,
    /// Record markings of which cell positions on the board have been called.
    marks: [[bool; C]; R],
    /// Tracks the final score. `None` if it has not reached the winning state just yet.
    score: Option<T>,
}

impl<T, const R: usize, const C: usize> BoardChecker<'_, T, R, C>
where
    T: PrimInt,
{
    /// Marks a called lot on the bingo board and finalizes the score if winning state has been reached.
    /// Subsequent marks after the first winning does not alter the bingo board markings.
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

    /// Checks whether a given row has achieved the winning state.
    fn check_row_winning(&self, row: usize) -> bool {
        (0..C).all(|j| self.marks[row][j])
    }

    /// Checks whether a given column has achieved the winning state.
    fn check_col_winning(&self, col: usize) -> bool {
        (0..R).all(|i| self.marks[i][col])
    }

    /// Computes the sum of unmarked numbers on the bingo board.
    fn sum_unmarked(&self) -> T
    where
        T: Sum,
    {
        iproduct!(0..R, 0..C)
            .filter(|&(i, j)| !self.marks[i][j])
            .map(|(i, j)| self.board.numbers[i][j])
            .sum()
    }
}

/// The result from playing a bingo game with a sequence of lots
#[derive(Debug, Clone, Copy)]
struct PlayResult<T> {
    /// Final score of the bingo board; `None` if the board has reached the winning state
    score: Option<T>,
    /// The number of called lots until the board has reached a winning state.
    /// If the winning state has never been reached, it still stores the total number of lots called.
    rounds_played: usize,
}
