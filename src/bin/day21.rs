//! Day 21: Dirac Dice, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/21>
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{ensure, Context};
use clap::Parser;
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser::Cli;
use aoc2021::hashing::HashMap;
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

    // Part 2: Dirac game
    let part2_answer = {
        let game_config = GameConfig::new(10, 21, 3);
        let game_result = simulate_dirac_game(&player_data, &game_config, [1, 2, 3].as_slice());
        u64::max(game_result.winning_counts[0], game_result.winning_counts[1])
    };
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    /// Iterator that generates a sequence of all possible combinations of in-game player statistics
    /// starting from lowest scores first.
    fn stats_space(&self) -> Vec<PlayerStat> {
        iproduct!(0..self.score_goal, 1..=self.board_size)
            .map(|(score, pos)| PlayerStat { pos, score })
            .collect()
    }

    /// Computes the step ladders: a distribution of moving steps by their likelihood
    fn ladders(&self, dice_faces: &[u64]) -> Vec<Ladder> {
        let counts = (0..self.rolls_per_turn)
            .map(|_| dice_faces.iter())
            .multi_cartesian_product()
            .map(|v| v.into_iter().sum::<u64>())
            .counts();
        counts
            .keys()
            .sorted()
            .map(|steps| Ladder {
                steps: *steps,
                freq: counts[steps] as u64,
            })
            .collect()
    }
}

/// Moving step ladders
#[derive(Debug, Clone)]
struct Ladder {
    steps: u64,
    freq: u64,
}

/// Player identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    One,
    Two,
}

impl Player {
    /// Converts player object as an index
    fn as_index(&self) -> usize {
        match self {
            Player::One => 0,
            Player::Two => 1,
        }
    }

    /// Obtains another player
    fn other(&self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

/// Final result for the simplified version of the game of dice
#[derive(Debug, Clone)]
struct SimplifiedGameResult {
    player_stats: [PlayerStat; 2],
    winning_player: Player,
    total_rolls: u64,
}

impl SimplifiedGameResult {
    /// Obtains the losing player statistics
    fn losing_player(&self) -> &PlayerStat {
        &self.player_stats[self.winning_player.other().as_index()]
    }
}

/// Final result for the Dirac game of dice
#[derive(Debug, Clone)]
struct DiracGameResult {
    winning_counts: [u64; 2],
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
    let mut roll = |total_rolls: &mut u64| -> u64 {
        *total_rolls += game_config.rolls_per_turn as u64;
        (0..game_config.rolls_per_turn)
            .map(|_| dice_rolls.next().unwrap())
            .sum()
    };

    let mut total_rolls: u64 = 0;
    for next_player in [Player::One, Player::Two].into_iter().cycle() {
        let next_stat = &mut player_stats[next_player.as_index()];
        let move_steps = roll(&mut total_rolls);
        *next_stat = next_stat.get_updated(move_steps, game_config);
        if next_stat.score >= game_config.score_goal {
            return SimplifiedGameResult {
                player_stats,
                winning_player: next_player,
                total_rolls,
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
    let mut table: HashMap<(PlayerStat, PlayerStat, Player), u64> = HashMap::default();
    table.insert(
        (
            player_data[0].new_game(),
            player_data[1].new_game(),
            Player::One,
        ),
        1,
    );

    let ladders = game_config.ladders(dice_faces);
    let stats_space = game_config.stats_space();
    let mut winning_counts = [0; 2];
    for (p1_stat, p2_stat, player_index) in iproduct!(
        stats_space.iter(),
        stats_space.iter(),
        [Player::One, Player::Two]
    ) {
        let index = (p1_stat.clone(), p2_stat.clone(), player_index);
        let count = match table.get(&index) {
            None => continue,
            Some(&v) => v,
        };

        // eprintln!(
        //     "{:?} {:?} {:?} => {:?}",
        //     p1_stat, p2_stat, player_index, count
        // );

        for ladder in ladders.iter() {
            let next_index = match player_index {
                Player::One => {
                    let p1_updated = p1_stat.get_updated(ladder.steps, game_config);
                    if p1_updated.score >= game_config.score_goal {
                        winning_counts[0] += ladder.freq * count;
                        continue;
                    }
                    (p1_updated, p2_stat.clone(), player_index.other())
                }
                Player::Two => {
                    let p2_updated = p2_stat.get_updated(ladder.steps, game_config);
                    if p2_updated.score >= game_config.score_goal {
                        winning_counts[1] += ladder.freq * count;
                        continue;
                    }
                    (p1_stat.clone(), p2_updated, player_index.other())
                }
            };
            *table.entry(next_index).or_insert(0) += ladder.freq * count;
        }
    }

    DiracGameResult { winning_counts }
}
