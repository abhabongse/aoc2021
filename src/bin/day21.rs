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
    let Input { player_data } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Deterministic game
    let part1_answer = {
        let game_config = GameConfig::new(10, 1000, 3);
        let game_result =
            simulate_deterministic_game(&player_data, &game_config, (1..=1000).cycle());
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
    /// Initial states of both players in a game of dice
    player_data: [PlayerInitState; 2],
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let p1_init_state: PlayerInitState = lines
            .next()
            .context("expected first line input")??
            .parse()?;
        ensure!(p1_init_state.id == 1);
        let p2_init_state: PlayerInitState = lines
            .next()
            .context("expected first line input")??
            .parse()?;
        ensure!(p2_init_state.id == 2);
        let player_data = [p1_init_state, p2_init_state];
        Ok(Input { player_data })
    }
}

/// Initial state of a player in a game of dice
#[derive(Debug, Clone)]
struct PlayerInitState {
    /// Player's ID
    id: u8,
    /// Player's starting position
    pos: u64,
}

impl PlayerInitState {
    /// Creates the player statistics tracking object for a new game.
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

/// Current statistic of a player in a game of dice
#[derive(Debug, Clone)]
struct PlayerStat {
    /// Current position of the player
    pos: u64,
    /// Current score of the player
    score: u64,
}

impl PlayerStat {
    /// Obtains the statistics of the player as a new struct
    /// based on the number of move steps and game configuration.
    fn get_updated(&self, move_steps: u64, game_config: &GameConfig) -> Self {
        let pos = match (self.pos + move_steps) % game_config.board_size {
            0 => game_config.board_size,
            pos => pos,
        };
        let score = self.score + pos;
        PlayerStat { pos, score }
    }
}

/// Configuration data for a game of dice
#[derive(Debug, Clone)]
struct GameConfig {
    /// Size of the board; board spaces are labeled from 1 through `board_size`
    /// and wraps around in the clockwise order
    board_size: u64,
    /// Minimum score required to be declared as winning player
    score_goal: u64,
    /// Number of dice rolls per player's turn
    rolls_per_turn: usize,
}

impl GameConfig {
    /// Creates a new configuration for a game of dice.
    fn new(board_size: u64, score_goal: u64, rolls_per_turn: usize) -> Self {
        GameConfig {
            board_size,
            score_goal,
            rolls_per_turn,
        }
    }
}

/// Final result for the simplified version of the game of dice
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

/// Simulates the simplified version of the game of dice
/// using the given initial `player_data`, the `game_config`,
/// and the infinite sequence of deterministic `dice_roll` outcomes.
/// Note that if the `dice_roll` was exhausted before the game ends then this function will panic.
/// Otherwise it returns the final result of the game.
fn simulate_deterministic_game(
    player_data: &[PlayerInitState; 2],
    game_config: &GameConfig,
    mut dice_rolls: impl Iterator<Item = u64>,
) -> SimplifiedGameResult {
    let mut player_stats = [player_data[0].new_game(), player_data[1].new_game()];
    let mut roll = || {
        let mut total = 0;
        for _ in 0..game_config.rolls_per_turn {
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
        *next_player = next_player.get_updated(move_steps, game_config);
        if next_player.score >= game_config.score_goal {
            return SimplifiedGameResult {
                player_stats,
                winning_player_index: next_player_index,
                total_rolls: (turn_count * game_config.rolls_per_turn) as u64,
            };
        }
    }
    unreachable!()
}

/// Simulates the Dirac (multiple universe explosion) version of the game of dice
/// using the given initial `player_data`, the `game_config`,
/// and a sequence of all possible outcomes of `dice_faces` after each roll.
fn simulate_dirac_game(
    player_data: &[PlayerInitState; 2],
    game_config: &GameConfig,
    dice_faces: &[u64],
) -> DiracGameResult {
    todo!()
}
