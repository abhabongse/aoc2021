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
    let part1_answer = {
        let game_result = simulate_deterministic_game(
            [p1_data.clone(), p2_data.clone()],
            10,
            1000,
            3,
            (1..=1000).cycle(),
        );
        game_result.losing_player().score * game_result.total_rolls
    };
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
    /// Updates player's current statistics and returns as new struct.
    fn get_updated(&self, move_steps: u64, board_size: u64) -> Self {
        let pos = match (self.pos + move_steps) % board_size {
            0 => board_size,
            pos => pos,
        };
        let score = self.score + pos;
        PlayerStat { pos, score }
    }
}

/// Final result for the simplified version of the dice game
#[derive(Debug, Clone)]
struct SimplifiedGameResult {
    player_stats: [PlayerStat; 2],
    winning_player_index: usize,
    total_rolls: u64,
}

impl SimplifiedGameResult {
    /// Obtains the losing player statistics
    fn losing_player(&self) -> &PlayerStat {
        &self.player_stats[1 - self.winning_player_index]
    }
}

/// Simulates the simplified version of the dice game using the provided `player_data`
/// with the specified `board_size`, `score_goal`, and the number of `rolls_per_turn`.
/// The board circular cells are labeled `1` through `board_size` respectively.
/// The infinite iterator `dice_roll` deterministically determines the sequence of dice roll outcomes
/// (the function will panic if the `dice_roll` runs out of items from the iterator).
fn simulate_deterministic_game<I>(
    player_data: [PlayerInitState; 2],
    board_size: u64,
    score_goal: u64,
    rolls_per_turn: usize,
    mut dice_rolls: I,
) -> SimplifiedGameResult
where
    I: Iterator<Item = u64>,
{
    let mut player_stats = [player_data[0].new_game(), player_data[1].new_game()];
    let mut roll = || {
        let mut total = 0;
        for _ in 0..rolls_per_turn {
            total += dice_rolls
                .next()
                .context("dice roll should be infinite")
                .unwrap();
        }
        total
    };

    for turn_count in 1.. {
        let next_player_index = (turn_count - 1) % 2;
        let next_player = &mut player_stats[next_player_index];
        let move_steps = roll();
        *next_player = next_player.get_updated(move_steps, board_size);
        if next_player.score >= score_goal {
            return SimplifiedGameResult {
                player_stats,
                winning_player_index: next_player_index,
                total_rolls: (turn_count * rolls_per_turn) as u64,
            };
        }
    }
    unreachable!()
}

fn simulate_dirac_game(
    p1_data: PlayerInitState,
    p2_data: PlayerInitState,
    board_size: u64,
    score_goal: u64,
    rolls_per_turn: usize,
    dice_faces: &[u64],
) -> u64 {
    todo!()
}
