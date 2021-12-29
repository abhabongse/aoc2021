//! Day 18: Snailfish, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/18>
use anyhow::anyhow;
use lazy_static::lazy_static;
use std::io::BufRead;

use aoc2021::argparser;
use aoc2021::snailfish::{ExprParser, Node};

lazy_static! {
    static ref EXPR_PARSER: ExprParser = ExprParser::new();
}

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { numbers } = Input::from_buffer(input_reader).expect("cannot parse input");

    for n in numbers {
        eprintln!("{:?}", n);
    }

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
    numbers: Vec<Node>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut numbers = Vec::new();
        for line in reader.lines() {
            let line = line?;
            numbers.push(
                EXPR_PARSER
                    .parse(line.as_str())
                    .map_err(|_| anyhow!("cannot parse line: '{}'", line.escape_default()))?,
            )
        }
        Ok(Input { numbers })
    }
}
