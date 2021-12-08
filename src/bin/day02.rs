//! Day 2: Dive!, Advent of Code 2021
//! https://adventofcode.com/2021/day/2
use std::io::BufRead;
use std::str::FromStr;

use anyhow::bail;

use aoc2021::argparser;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.create_reader().expect("cannot open file");
    let commands = parse_input(input_reader).expect("cannot parse input");

    // Part 1: naïve submarine navigation
    let p1_submarine =
        commands
            .iter()
            .fold(Vector2D::default(), |Vector2D { x, y }, cmd| match cmd {
                Command::Forward(dist) => Vector2D { x: x + dist, y },
                Command::Down(dist) => Vector2D { x, y: y + dist },
                Command::Up(dist) => Vector2D { x, y: y - dist },
            });
    println!("Part 1 answer: {}", p1_submarine.pos_product());

    // Part 2: submarine navigation with aim attribute
    let p2_submarine = commands.iter().fold(
        SubmarineStatus::default(),
        |SubmarineStatus { pos, aim }, cmd| match cmd {
            Command::Forward(dist) => SubmarineStatus {
                pos: Vector2D {
                    x: pos.x + dist,
                    y: pos.y + aim * dist,
                },
                aim,
            },
            Command::Down(dist) => SubmarineStatus {
                pos,
                aim: aim + dist,
            },
            Command::Up(dist) => SubmarineStatus {
                pos,
                aim: aim - dist,
            },
        },
    );
    println!("Part 2 answer: {}", p2_submarine.pos.pos_product());
}

/// Parses the submarine commands (program input) as a vector of [`Command`] struct.
fn parse_input<R: BufRead>(reader: R) -> anyhow::Result<Vec<Command>> {
    reader.lines().map(|line| line?.parse()).collect()
}

/// Submarine navigation commands
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

/// Two-dimensional point.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct Vector2D {
    /// x-coordinate
    x: i64,
    /// y-coordinate
    y: i64,
}

impl Vector2D {
    /// The product of x- and y-coordinates.
    fn pos_product(&self) -> i64 {
        self.x * self.y
    }
}

/// Submarine status: positions and aim
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct SubmarineStatus {
    /// Position of the submarine
    pos: Vector2D,
    /// Aim propulsion attribute
    aim: i64,
}
