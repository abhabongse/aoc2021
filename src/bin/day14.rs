//! Day 14: Extended Polymerization, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/14>
use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::argparser;
use aoc2021::quickparse::QuickParse;
use aoc2021::try_collect::TryCollectArray;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let Input {
        template,
        ins_rules,
    } = Input::from_buffer(input_reader).expect("cannot parse input");

    eprintln!("{:?}", template);
    eprintln!("{:?}", ins_rules);

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
    template: String,
    ins_rules: Vec<InsertionRule>,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let template = match lines.next() {
            Some(Ok(s)) => s.trim().to_string(),
            _ => bail!("expected the template string on the first line"),
        };
        match lines.next() {
            Some(Ok(s)) if s.trim().is_empty() => {}
            _ => bail!("expected an empty line after the first line"),
        }
        let mut ins_rules = Vec::new();
        for line in lines {
            let line = line.context("cannot read a line of string")?;
            ins_rules.push(line.quickparse()?);
        }
        Ok(Input {
            template,
            ins_rules,
        })
    }
}

/// Polymerization insertion rules
#[derive(Debug, Clone)]
struct InsertionRule {
    /// Pair of characters to capture
    pattern: (char, char),
    /// Character to insert between the pair of pattern characters
    insert_char: char,
}

impl FromStr for InsertionRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s*(\w\w)\s+->\s+(\w)\s*").unwrap();
        }
        // let RE: Regex = Regex::new(r"\s*(\w\w)\s+->\s+(\w)\s*").unwrap();
        let captures = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid insertion rule: {}", s))?;
        let [fst, snd] = captures[1].chars().try_collect_exact_array()?;
        let [insert_char] = captures[2].chars().try_collect_exact_array()?;
        let pattern = (fst, snd);
        Ok(InsertionRule {
            pattern,
            insert_char,
        })
    }
}
