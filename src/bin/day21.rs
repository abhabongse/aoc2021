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
    let Input { player_1, player_2 } =
        Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: TODO
    let p1_answer = 0;
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    player_1: u8,
    player_2: u8,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let player_1: PlayerInitState = lines
            .next()
            .context("expected first line input")??
            .parse()?;
        ensure!(player_1.id == 1);
        let player_2: PlayerInitState = lines
            .next()
            .context("expected first line input")??
            .parse()?;
        ensure!(player_2.id == 2);
        Ok(Input {
            player_1: player_1.pos,
            player_2: player_2.pos,
        })
    }
}

/// Initial state of a player
#[derive(Debug, Clone)]
struct PlayerInitState {
    /// Player ID
    id: u8,
    /// Player starting position
    pos: u8,
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
