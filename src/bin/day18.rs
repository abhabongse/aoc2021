//! Day 18: Snailfish, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/18>
use std::fmt::{Debug, Display, Formatter};
use std::io::BufRead;
use std::iter::once;
use std::ops::Add;

use anyhow::anyhow;
use itertools::{chain, Itertools};
use lazy_static::lazy_static;

use aoc2021::argparser::InputSrc;
use aoc2021::snailfish::{Node, SnailfishParser};

lazy_static! {
    static ref PARSER: SnailfishParser = SnailfishParser::new();
}

/// Main program
fn main() {
    let input_src = InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input { numbers } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Serialize snailfish numbers into stack-oriented representation
    let numbers = numbers.iter().map(SerializedSnailfish::from).collect_vec();

    // Part 1: Sum of all numbers
    let p1_answer = {
        let result = numbers[1..]
            .iter()
            .fold(numbers[0].clone(), |acc, n| (&acc + n).reduce());
        println!("Final result: {}", result);
        result.magnitude()
    };
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Largest sum of a pair
    let p2_answer = numbers
        .iter()
        .permutations(2)
        .map(|v| (v[0] + v[1]).reduce().magnitude())
        .max()
        .expect("empty seq of numbers");
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

/// Stack-oriented representation of a snailfish number serialized in sequence of [`Element`]
#[derive(Clone)]
struct SerializedSnailfish(Vec<Element>);

impl SerializedSnailfish {
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
                Element::LBracket => level += 1,
                Element::RBracket => level -= 1,
                Element::Value(_) if level >= 5 => {
                    pivot = Some(pos - 1);
                    break;
                }
                _ => (),
            }
        }
        pivot.map(|pos| {
            let (fst, snd) = match self.0[pos..pos+4] {
                [Element::LBracket, Element::Value(fst), Element::Value(snd), Element::RBracket] => (fst, snd),
                _ => panic!("invalid serialization of snailfish number"),
            };
            let elements = chain!(self.0[..pos].iter(), once(&Element::Value(0)), self.0[pos+4..].iter());
            let mut elements = elements.copied().collect_vec();
            for elem in elements[..pos].iter_mut().rev() {
                if elem.is_value() {
                    *elem = elem.map(|v| v + fst);
                    break;
                }
            }
            for elem in elements[pos+1..].iter_mut() {
                if elem.is_value() {
                    *elem = elem.map(|v| v + snd);
                    break;
                }
            }
            SerializedSnailfish(elements)
        })
    }

    /// Split the snailfish itself, if possible.
    fn split(&self) -> Option<Self> {
        let pivot = self
            .0
            .iter()
            .find_position(|elem| matches!(elem, Element::Value(v) if *v >= 10));
        pivot.map(|(pos, elem)| {
            let value = elem.unwrap_value();
            let fst = value / 2;
            let snd = value - fst;
            let new_elements = [
                Element::LBracket,
                Element::Value(fst),
                Element::Value(snd),
                Element::RBracket,
            ];
            let elements = chain!(
                self.0[..pos].iter(),
                new_elements.iter(),
                self.0[pos + 1..].iter()
            );
            let elements = elements.copied().collect_vec();
            SerializedSnailfish(elements)
        })
    }

    /// Magnitude of the snailfish
    fn magnitude(&self) -> i64 {
        let mut stack = Vec::new();
        for elem in self.0.iter() {
            match elem {
                Element::LBracket => (),
                Element::RBracket => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(3 * left + 2 * right);
                }
                Element::Value(v) => stack.push(*v),
            }
        }
        let result = stack.pop().unwrap();
        assert!(stack.is_empty());
        result
    }
}

impl Add<&SerializedSnailfish> for &SerializedSnailfish {
    type Output = SerializedSnailfish;

    fn add(self, rhs: &SerializedSnailfish) -> Self::Output {
        let mut result = Vec::from([Element::LBracket]);
        result.extend_from_slice(self.0.as_slice());
        result.extend_from_slice(rhs.0.as_slice());
        result.push(Element::RBracket);
        SerializedSnailfish(result)
    }
}

impl From<&Node> for SerializedSnailfish {
    fn from(tree: &Node) -> Self {
        fn process(acc: &mut Vec<Element>, node: &Node) {
            match node {
                Node::Branch(left, right) => {
                    acc.push(Element::LBracket);
                    process(acc, left);
                    process(acc, right);
                    acc.push(Element::RBracket);
                }
                Node::Leaf(value) => acc.push(Element::Value(*value)),
            }
        }
        let mut acc = Vec::new();
        process(&mut acc, tree);
        SerializedSnailfish(acc)
    }
}

impl Display for SerializedSnailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let stream = chain!(once(Element::LBracket), self.0.iter().copied());
        for (prev, curr) in stream.tuple_windows() {
            if prev != Element::LBracket && curr != Element::RBracket {
                write!(f, ",")?;
            }
            match curr {
                Element::LBracket => write!(f, "[")?,
                Element::RBracket => write!(f, "]")?,
                Element::Value(v) => write!(f, "{}", v)?,
            }
        }
        Ok(())
    }
}

/// Elements in stack-oriented representation of a snailfish number
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    /// Left (open) bracket
    LBracket,
    /// Right (closed) bracket
    RBracket,
    /// Integer value
    Value(i64),
}

impl Element {
    /// Checks if the element contains an integer value.
    fn is_value(self) -> bool {
        matches!(self, Element::Value(_))
    }

    /// Returns the integer value contained within [`Element::Value`].
    /// This function panics if the element is not a value.
    fn unwrap_value(self) -> i64 {
        match self {
            Element::Value(v) => v,
            _ => panic!("{:?} not a value element", self),
        }
    }

    /// Replaces the value within [`Element::Value`] using the specified mapping function.
    /// Nothing changes if the element is not a value.
    fn map<F>(self, f: F) -> Element
    where
        F: FnOnce(i64) -> i64,
    {
        match self {
            Element::Value(v) => Element::Value(f(v)),
            _ => self,
        }
    }
}
