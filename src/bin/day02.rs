//! Day 2: Dive!, Advent of Code 2021
//! https://adventofcode.com/2021/day/2
use anyhow::bail;
use aoc2021::argparser;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let input_file = std::env::args().nth(1);
    let reader = argparser::reader_from_file(input_file.as_deref());
    let input = parse_input(reader).expect("cannot parse input");

    let p1_submarine =
        input.iter().fold(
            SubmarinePos::default(),
            |SubmarinePos { x, y }, cmd| match cmd {
                Command::Forward(dist) => SubmarinePos { x: x + dist, y },
                Command::Down(dist) => SubmarinePos { x, y: y + dist },
                Command::Up(dist) => SubmarinePos { x, y: y - dist },
            },
        );
    println!("Part 1 answer: {}", p1_submarine.x * p1_submarine.y);

    let p2_submarine = input.iter().fold(
        SubmarineStatus::default(),
        |SubmarineStatus { x, y, aim }, cmd| match cmd {
            Command::Forward(dist) => SubmarineStatus {
                x: x + dist,
                y: y + aim * dist,
                aim,
            },
            Command::Down(dist) => SubmarineStatus {
                x,
                y,
                aim: aim + dist,
            },
            Command::Up(dist) => SubmarineStatus {
                x,
                y,
                aim: aim - dist,
            },
        },
    );
    println!("Part 2 answer: {}", p2_submarine.x * p2_submarine.y);
}

fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<Command>> {
    reader.lines().map(|line| line.unwrap().parse()).collect()
}

/// Submarine commands: move instructions
#[derive(Debug, Clone, Eq, PartialEq)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_ascii_whitespace().collect();
        match tokens[..] {
            ["forward", param] => Ok(Command::Forward(param.parse()?)),
            ["down", param] => Ok(Command::Down(param.parse()?)),
            ["up", param] => Ok(Command::Up(param.parse()?)),
            [] => bail!("empty command"),
            _ => bail!("invalid command: {}", s.trim()),
        }
    }
}

/// Submarine positions
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct SubmarinePos {
    x: i64,
    y: i64,
}

/// Submarine status: positions and aim
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct SubmarineStatus {
    x: i64,
    y: i64,
    aim: i64,
}