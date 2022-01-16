//! Day 21: Dirac Dice, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/21>
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::Cli;
use aoc2021::parsing::QuickParse;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { p1_stats, p2_stats } =
        Input::from_buffer(input_reader).expect("cannot parse input");

    eprintln!("Player {} stats: {:?}", p1_stats.id, p1_stats);
    eprintln!("Player {} stats: {:?}", p2_stats.id, p2_stats);

    // Part 1: TODO
    let part1_answer = {
        let game = GameState::new(10, 1000, p1_stats.clone(), p2_stats.clone());
        game.simulate((1..=1000).cycle(), 3)
    };
    println!("Part 1 answer: {}", part1_answer);

    // Part 2: TODO
    let part2_answer = 0;
    println!("Part 2 answer: {}", part2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Player 1 statistics
    p1_stats: PlayerStats,
    /// Player 2 statistics
    p2_stats: PlayerStats,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let p1_stats: PlayerStats = lines
            .next()
            .context("expected first line input")??
            .parse()?;
        let p2_stats: PlayerStats = lines
            .next()
            .context("expected first line input")??
            .parse()?;
        Ok(Input { p1_stats, p2_stats })
    }
}

/// A state of Dirac Dice game
#[derive(Debug, Clone)]
struct GameState {
    /// Size of the board, with each cell labeling `1` through `board_size`
    board_size: u64,
    /// Scores required to win the game
    goal: u64,
    /// Player statistics
    player_stats: [PlayerStats; 2],
}

impl GameState {
    /// Creates a new game
    fn new(board_size: u64, goal: u64, p1_stats: PlayerStats, p2_stats: PlayerStats) -> Self {
        GameState {
            board_size,
            goal,
            player_stats: [p1_stats, p2_stats],
        }
    }

    /// Simulate the game with the specified dice rolls until a player wins.
    /// It returns the product of the losing player's score with the number of times the die is rolled.
    fn simulate<I>(&self, mut dice_rolls: I, rolls_per_turn: usize) -> u64
    where
        I: Iterator<Item = u64>,
    {
        let mut next_player = self.player_stats[0].clone();
        let mut other_player = self.player_stats[1].clone();
        for turn_count in 1.. {
            let move_steps: u64 = dice_rolls.by_ref().take(rolls_per_turn).sum();
            next_player.pos = match (next_player.pos + move_steps) % self.board_size {
                0 => self.board_size,
                pos => pos,
            };
            next_player.score += next_player.pos;

            if next_player.score >= self.goal {
                return other_player.score * turn_count * rolls_per_turn as u64;
            }

            std::mem::swap(&mut next_player, &mut other_player);
        }
        unreachable!()
    }
}

/// Statistics of a player in the Dirac Dice game
#[derive(Debug, Clone)]
struct PlayerStats {
    id: u8,
    pos: u64,
    score: u64,
}

impl FromStr for PlayerStats {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PLAYER_INITIAL_STATE: Regex =
                Regex::new(r"(?i)player\s+(\d+)\s+starting\s+position:\s*(\d+)").unwrap();
        }
        let captures = PLAYER_INITIAL_STATE
            .captures(s)
            .context("invalid input line format")?;
        Ok(PlayerStats {
            id: captures[1].quickparse()?,
            pos: captures[2].quickparse()?,
            score: 0,
        })
    }
}
