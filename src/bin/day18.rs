//! Day 18: Snailfish, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/18>
use std::fmt::{Debug, Formatter};
use std::io::BufRead;
use std::ops::Add;

use anyhow::anyhow;
use itertools::Itertools;
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
        let result = numbers
            .iter()
            .map(SerialSnailfish::from)
            .fold1(|ref acc, ref n| (acc + n).reduce())
            .expect("empty seq of numbers");
        // eprintln!("{:?}", result);
        result.magnitude()
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

#[derive(Clone)]
struct SerialSnailfish(Vec<Element>);

impl SerialSnailfish {
    /// Obtains the reduced form of the snailfish itself.
    fn reduce(&self) -> Self {
        let mut fish = self.clone();
        loop {
            if let Some(exploded_fish) = fish.explode() {
                fish = exploded_fish;
                continue;
            }
            if let Some(splitted_fish) = fish.split() {
                fish = splitted_fish;
                continue;
            }
            return fish;
        }
    }

    /// Explode the snailfish itself, if possible.
    fn explode(&self) -> Option<Self> {
        let mut level: usize = 0;
        let mut pivot = None;
        for (pos, elem) in self.0.iter().enumerate() {
            match elem {
                Element::IncLevel => level += 1,
                Element::DecLevel => level -= 1,
                Element::Value(_) if level >= 5 => {
                    pivot = Some(pos - 1);
                    break;
                }
                _ => (),
            }
        }
        pivot.map(|pos| {
            assert_eq!(self.0[pos], Element::IncLevel);
            let fst = self.0[pos + 1].unwrap_value();
            let snd = self.0[pos + 2].unwrap_value();
            assert_eq!(self.0[pos + 3], Element::DecLevel);
            let mut left_half = self.0[..pos].to_vec();
            for elem in left_half.iter_mut().rev() {
                if elem.is_value() {
                    *elem = elem.map(|v| v + fst);
                    break;
                }
            }
            let mut right_half = self.0[pos + 4..].to_vec();
            for elem in right_half.iter_mut() {
                if elem.is_value() {
                    *elem = elem.map(|v| v + snd);
                    break;
                }
            }
            let mut elements = Vec::with_capacity(self.0.len() - 3);
            elements.append(&mut left_half);
            elements.push(Element::Value(0));
            elements.append(&mut right_half);
            SerialSnailfish(elements)
        })
    }

    /// Split the snailfish itself, if possible.
    fn split(&self) -> Option<Self> {
        let pivot = self
            .0
            .iter()
            .find_position(|elem| matches!(elem, Element::Value(value) if *value >= 10));
        pivot.map(|(pos, elem)| {
            let value = elem.unwrap_value();
            let fst = value / 2;
            let snd = value - fst;
            let mut elements = Vec::with_capacity(self.0.len() + 3);
            elements.extend_from_slice(&self.0[..pos]);
            elements.extend([
                Element::IncLevel,
                Element::Value(fst),
                Element::Value(snd),
                Element::DecLevel,
            ]);
            elements.extend_from_slice(&self.0[pos + 1..]);
            SerialSnailfish(elements)
        })
    }

    /// Magnitude of the snailfish
    fn magnitude(&self) -> i64 {
        let mut stack = Vec::new();
        for elem in self.0.iter() {
            match elem {
                Element::IncLevel => (),
                Element::DecLevel => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(3 * left + 2 * right);
                }
                Element::Value(value) => stack.push(*value),
            }
        }
        let result = stack.pop().unwrap();
        assert!(stack.is_empty());
        result
    }
}

impl Add<&SerialSnailfish> for &SerialSnailfish {
    type Output = SerialSnailfish;

    fn add(self, rhs: &SerialSnailfish) -> Self::Output {
        let mut result = Vec::from([Element::IncLevel]);
        result.extend_from_slice(self.0.as_slice());
        result.extend_from_slice(rhs.0.as_slice());
        result.push(Element::DecLevel);
        SerialSnailfish(result)
    }
}

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

impl Debug for SerialSnailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut prev = Element::IncLevel;
        for curr in self.0.iter() {
            if !matches!(prev, Element::IncLevel) && !matches!(curr, Element::DecLevel) {
                write!(f, ",")?;
            }
            match curr {
                Element::IncLevel => write!(f, "[")?,
                Element::DecLevel => write!(f, "]")?,
                Element::Value(value) => write!(f, "{}", value)?,
            };
            prev = *curr;
        }
        Ok(())
    }
}

/// Element of a stack-based serialized representation of snailfish number
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    IncLevel,
    DecLevel,
    Value(i64),
}

impl Element {
    fn is_value(self) -> bool {
        matches!(self, Element::Value(_))
    }

    fn unwrap_value(self) -> i64 {
        match self {
            Element::Value(value) => value,
            _ => panic!("{:?} not a value element", self),
        }
    }

    fn map<F>(self, f: F) -> Element
    where
        F: FnOnce(i64) -> i64,
    {
        match self {
            Element::Value(value) => Element::Value(f(value)),
            _ => self,
        }
    }
}
