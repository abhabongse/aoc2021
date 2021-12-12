//! Day 11: Dumbo Octopus, Advent of Code 2021
//! <https://adventofcode.com/2021/day/11>
use std::collections::HashMap;
use std::io::BufRead;

use anyhow::Context;
use itertools::Itertools;

use aoc2021::argparser;
use aoc2021::try_collect::TryCollectArray;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let graph = parse_input(input_reader).expect("cannot parse input");

    // Part 1: visiting each small cave at most once
    let p1_answer = {
        let mut count: usize = 0;
        graph.exhaustive_traverse(
            "start",
            "end",
            // Acceptable cases:
            // 1.  the next node is a big cave (containing uppercase letters), or
            // 2.  the path so far does _not_ contain such next node
            |next, path| next.chars().any(char::is_uppercase) || !path.contains(&next),
            |_path| {
                // eprintln!("=> {}", _path.join(", "));
                count += 1
            },
        );
        count
    };
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: visiting each small cave at most once, except for one that is allowed up to twice
    //         but excluding the start and the end
    let p2_answer = {
        let mut count = 0;
        graph.exhaustive_traverse(
            "start",
            "end",
            // Acceptable cases (the first two are the same as part 1):
            // 1.  the next node is a big cave (containing uppercase letters), or
            // 2.  the path so far does _not_ contain such next node, or
            // 3.  the next node does _not_ go back to "start" and all previous small caves are unique visits
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

/// Parses the graph data (list of edges) as an adjacency list.
fn parse_input<BR: BufRead>(reader: BR) -> anyhow::Result<Graph> {
    let mut graph = Graph::new();
    for line in reader.lines() {
        let line = line.context("cannot read a line of string")?;
        let [u, v] = line.trim().split('-').try_collect_exact_array()?;
        graph.add_edge(u, v);
        graph.add_edge(v, u);
    }
    Ok(graph)
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
            adjlists: HashMap::new(),
        }
    }

    /// Add a directed edge from node `u` to node `v`.
    ///
    /// # Implementation Note
    /// I am not satisfied with my current optimizations
    /// to avoid duplicated allocations of identical string.
    /// TODO: Ideally, I should figure out how to approach this.
    ///       -  Possibility #1: using reference counting [`std::rc::Rc`]
    ///       -  Possibility #2: Look up how to manage lifetimes within structs
    ///          when one member contains a reference to the other member
    fn add_edge<T: AsRef<str>>(&mut self, u: T, v: T) {
        let u = u.as_ref().to_string();
        let v = v.as_ref().to_string();
        self.adjlists.entry(u).or_insert_with(Vec::new).push(v);
    }

    /// Exhaustive path searching from `start` to `end`.
    /// Before walking onto each next node, the predicate `decide_should_walk` decides
    /// whether to proceed based on the next node and the path walked so far from the `start`.
    /// For each time a finished path from `start` to `end` is found,
    /// the function `process_finished_path` is invoked on such path for further processing.
    fn exhaustive_traverse<T: AsRef<str>, P, F>(
        &self,
        start: T,
        end: T,
        mut decide_should_walk: P,
        mut process_finished_path: F,
    ) where
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
