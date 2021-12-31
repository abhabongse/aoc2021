//! Day 2: Dive!, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/2>
use std::io::BufRead;
use std::str::FromStr;

use anyhow::bail;

use aoc2021::argparser::InputSrc;
use aoc2021::parsing::QuickParse;

/// Main program
fn main() {
    let input_src = InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { commands } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Naïve submarine navigation
    let p1_submarine = commands.iter().fold(SubmarinePos::default(), |pos, cmd| {
        next_submarine_pos(&pos, cmd)
    });
    println!("Part 1 answer: {}", p1_submarine.pos_product());

    // Part 2: Submarine navigation with aim attribute
    let p2_submarine = commands
        .iter()
        .fold(SubmarineStatus::default(), |status, cmd| {
            next_submarine_status(&status, cmd)
        });
    println!("Part 2 answer: {}", p2_submarine.pos.pos_product());
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// List of submarine commands
    commands: Vec<Command>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut commands = Vec::new();
        for line in reader.lines() {
            commands.push(line?.quickparse()?);
        }
        Ok(Input { commands })
    }
}

/// Submarine navigation commands
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_ascii_whitespace().collect();
        let cmd = match tokens[..] {
            ["forward", param] => Command::Forward(param.parse()?),
            ["down", param] => Command::Down(param.parse()?),
            ["up", param] => Command::Up(param.parse()?),
            [] => bail!("empty command"),
            _ => bail!("invalid command: {}", s.trim()),
        };
        Ok(cmd)
    }
}

/// Two-dimensional position of the submarine
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct SubmarinePos {
    /// Position in x-coordinate
    x: i64,
    /// Position in y-coordinate
    y: i64,
}

impl SubmarinePos {
    /// The product of x- and y-coordinates.
    fn pos_product(&self) -> i64 {
        self.x * self.y
    }
}

/// Submarine status at a given time.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct SubmarineStatus {
    /// Position of the submarine.
    pos: SubmarinePos,
    /// Aim propulsion attribute.
    aim: i64,
}

/// Computes next submarine position based on naïve understanding.
fn next_submarine_pos(pos: &SubmarinePos, cmd: &Command) -> SubmarinePos {
    match cmd {
        Command::Forward(dist) => SubmarinePos {
            x: pos.x + dist,
            y: pos.y,
        },
        Command::Down(dist) => SubmarinePos {
            x: pos.x,
            y: pos.y + dist,
        },
        Command::Up(dist) => SubmarinePos {
            x: pos.x,
            y: pos.y - dist,
        },
    }
}

/// Computes next submarine status based on correct understanding.
fn next_submarine_status(status: &SubmarineStatus, cmd: &Command) -> SubmarineStatus {
    match cmd {
        Command::Forward(dist) => SubmarineStatus {
            pos: SubmarinePos {
                x: status.pos.x + dist,
                y: status.pos.y + status.aim * dist,
            },
            aim: status.aim,
        },
        Command::Down(dist) => SubmarineStatus {
            pos: status.pos,
            aim: status.aim + dist,
        },
        Command::Up(dist) => SubmarineStatus {
            pos: status.pos,
            aim: status.aim - dist,
        },
    }
}
