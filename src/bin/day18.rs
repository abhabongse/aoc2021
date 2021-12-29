//! Day 18: Snailfish, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/18>
use std::fmt::{Debug, Formatter};
use std::io::BufRead;

use anyhow::anyhow;
use lazy_static::lazy_static;

use aoc2021::argparser;
use aoc2021::snailfish::{Node, SnailfishParser};

lazy_static! {
    static ref PARSER: SnailfishParser = SnailfishParser::new();
}

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { numbers } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Add and reduce numbers
    let p1_answer = {
        let numbers: Vec<_> = numbers.iter().map(|n| SerialSnailfish::from(n)).collect();
        eprintln!("{:?}", numbers);
        0
    };
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
                PARSER
                    .parse(line.as_str())
                    .map_err(|_| anyhow!("cannot parse line: '{}'", line.escape_default()))?,
            )
        }
        Ok(Input { numbers })
    }
}

#[derive(Debug, Clone)]
struct SerialSnailfish(Vec<Element>);

impl From<&Node> for SerialSnailfish {
    fn from(tree: &Node) -> Self {
        fn process(acc: &mut Vec<Element>, node: &Node) {
            match node {
                Node::Branch(left, right) => {
                    acc.push(Element::IncLevel);
                    process(acc, left);
                    process(acc, right);
                    acc.push(Element::DecLevel);
                }
                Node::Leaf(value) => acc.push(Element::Value(*value)),
            }
        }
        let mut acc = Vec::new();
        process(&mut acc, tree);
        SerialSnailfish(acc)
    }
}

/// Element of a stack-based serialized representation of snailfish number
#[derive(Clone, Copy)]
enum Element {
    IncLevel,
    DecLevel,
    Value(i64),
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::IncLevel => write!(f, "+"),
            Element::DecLevel => write!(f, "-"),
            Element::Value(value) => write!(f, "{}", value),
        }
    }
}
