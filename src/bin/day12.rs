//! Day 12: Passage Pathing, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/12>
use std::io::{BufRead, BufReader};

use anyhow::Context;
use clap::Parser;
use itertools::Itertools;

use aoc2021::argparser::Cli;
use aoc2021::collect_array::CollectArray;
use aoc2021::hashing::HashMap;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input { graph } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Visiting each small cave at most once
    let p1_answer = {
        let mut count: usize = 0;
        graph.exhaustive_traverse(
            "start",
            "end",
            // Acceptable cases:
            // 1.  The next node is a big cave (containing uppercase letters), or
            // 2.  The path so far does _not_ contain such next node
            |next, path| next.chars().any(char::is_uppercase) || !path.contains(&next),
            |_path| {
                // eprintln!("=> {}", _path.join(", "));
                count += 1
            },
        );
        count
    };
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Visiting each small cave at most once,
    // except for one that is allowed up to twice
    // but excluding the start and the end
    let p2_answer = {
        let mut count = 0;
        graph.exhaustive_traverse(
            "start",
            "end",
            // Acceptable cases (the first two are the same as part 1):
            // 1.  The next node is a big cave (containing uppercase letters), or
            // 2.  The path so far does _not_ contain such next node, or
            // 3.  The next node does _not_ go back to "start"
            //     AND all previous small caves are unique visits (new!)
            |next, path| {
                next.chars().all(char::is_uppercase)
                    || !path.contains(&next)
                    || next.ne("start")
                        && path
                            .iter()
                            .filter(|prev| !prev.chars().all(char::is_uppercase))
                            .all_unique()
            },
            |_path| {
                // eprintln!("=> {}", _path.join(", "));
                count += 1
            },
        );
        count
    };
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Graph data, as adjacency lists
    graph: Graph,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut graph = Graph::new();
        for line in reader.lines() {
            let line = line.context("cannot read a line of string")?;
            let [u, v] = line.trim().split('-').collect_exact()?;
            graph.add_edge(u, v);
            graph.add_edge(v, u);
        }
        Ok(Input { graph })
    }
}

/// Graph data with adjacency list data structure.
#[derive(Debug, Clone)]
struct Graph {
    /// Adjacency list of edges outgoing from each node.
    adjlists: HashMap<String, Vec<String>>,
}

impl Graph {
    /// Constructs a new graph instance.
    fn new() -> Self {
        Graph {
            adjlists: HashMap::default(),
        }
    }

    /// Add a directed edge from node `u` to node `v`.
    ///
    /// # Implementation Note
    /// I am not satisfied with my current optimizations
    /// to avoid duplicated allocations of identical string.
    /// - TODO: Introduce remapping from string identifier to an integer
    fn add_edge<T>(&mut self, u: T, v: T)
    where
        T: AsRef<str>,
    {
        let u = u.as_ref().to_string();
        let v = v.as_ref().to_string();
        self.adjlists.entry(u).or_insert_with(Vec::new).push(v);
    }

    /// Exhaustive path searching from `start` to `end`.
    /// Before the function decides to queue up walking onto an adjacent node,
    /// the predicate `decide_should_walk` decides whether to proceed
    /// based on the identifier of such node, and the path walked so far from the `start`.
    /// Once and each time a finished path from `start` to `end` has been found,
    /// the function `process_finished_path` is invoked with such path for further processing.
    fn exhaustive_traverse<T, P, F>(
        &self,
        start: T,
        end: T,
        mut decide_should_walk: P,
        mut process_finished_path: F,
    ) where
        T: AsRef<str>,
        P: FnMut(&str, &[&str]) -> bool,
        F: FnMut(&[&str]),
    {
        #[derive(Debug, Eq, PartialEq)]
        enum Event<'a> {
            PreStack(&'a str),
            InStack(&'a str),
        }
        let start = start.as_ref();
        let end = end.as_ref();

        let mut event_stack = Vec::from([Event::PreStack(start)]);
        let mut depth_stack = Vec::new();
        while let Some(event) = event_stack.pop() {
            match event {
                Event::PreStack(curr) => {
                    event_stack.push(Event::InStack(curr));
                    depth_stack.push(curr);
                    if curr == end {
                        process_finished_path(depth_stack.as_slice());
                    } else {
                        for next in self.adjlists[curr].iter() {
                            if decide_should_walk(next, depth_stack.as_slice()) {
                                event_stack.push(Event::PreStack(next))
                            }
                        }
                    }
                }
                Event::InStack(curr) => {
                    assert_eq!(curr, depth_stack.pop().expect("must not be empty"));
                }
            }
        }
    }
}
