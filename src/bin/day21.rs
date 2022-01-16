//! Day 21: Dirac Dice, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/21>
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{ensure, Context};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::Cli;
use aoc2021::parsing::QuickParse;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { p1_data, p2_data } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Deterministic game
    let part1_answer = simulate_deterministic_game(
        p1_data.clone(),
        p2_data.clone(),
        10,
        1000,
        3,
        (1..=1000).cycle(),
    );
    println!("Part 1 answer: {}", part1_answer);

    // Part 2: TODO
    let part2_answer = 0;
    println!("Part 2 answer: {}", part2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Player 1 initial statistics
    p1_data: PlayerInitState,
    /// Player 2 initial statistics
    p2_data: PlayerInitState,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let p1_data: PlayerInitState = lines
            .next()
            .context("expected first line input")??
            .parse()?;
        ensure!(p1_data.id == 1);
        let p2_data: PlayerInitState = lines
            .next()
            .context("expected first line input")??
            .parse()?;
        ensure!(p2_data.id == 2);
        Ok(Input { p1_data, p2_data })
    }
}

/// Initial state of a player in the Dirac Dice game
#[derive(Debug, Clone)]
struct PlayerInitState {
    id: u8,
    pos: u64,
}

impl PlayerInitState {
    /// Creates the player stat at the start of a new game.
    fn new_game(&self) -> PlayerStat {
        PlayerStat {
            pos: self.pos,
            score: 0,
        }
    }
}

impl FromStr for PlayerInitState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PLAYER_INITIAL_STATE: Regex =
                Regex::new(r"(?i)player\s+(\d+)\s+starting\s+position:\s*(\d+)").unwrap();
        }
        let captures = PLAYER_INITIAL_STATE
            .captures(s)
            .context("invalid input line format")?;
        Ok(PlayerInitState {
            id: captures[1].quickparse()?,
            pos: captures[2].quickparse()?,
        })
    }
}

/// Current statistic of a player in the Dirac Dice game
#[derive(Debug, Clone)]
struct PlayerStat {
    /// Current position of the player
    pos: u64,
    /// Current score of the player
    score: u64,
}

impl PlayerStat {
    /// Updates player's current statistics.
    fn update(&mut self, move_steps: u64, board_size: u64) {
        self.pos = match (self.pos + move_steps) % board_size {
            0 => board_size,
            pos => pos,
        };
        self.score += self.pos;
    }
}

/// Simulates the simplified version of the Dirac Dice game
/// using the provided initial player statistics (namely `player1_stats` and `player2_stats`)
/// with the specified `board_size`, `score_goal`, and the number of `rolls_per_turn`.
/// The iterator `dice_rolls` deterministically determines the sequence of dice rolls.
/// This function returns the product of the losing player's score and the total number of dice rolls.
fn simulate_deterministic_game<I>(
    p1_data: PlayerInitState,
    p2_data: PlayerInitState,
    board_size: u64,
    score_goal: u64,
    rolls_per_turn: usize,
    mut dice_rolls: I,
) -> u64
where
    I: Iterator<Item = u64>,
{
    let mut next_player = p1_data.new_game();
    let mut other_player = p2_data.new_game();
    for turn_count in 1.. {
        let move_steps: u64 = dice_rolls.by_ref().take(rolls_per_turn).sum();
        next_player.update(move_steps, board_size);
        if next_player.score >= score_goal {
            return other_player.score * turn_count * rolls_per_turn as u64;
        }
        std::mem::swap(&mut next_player, &mut other_player);
    }
    unreachable!()
}
