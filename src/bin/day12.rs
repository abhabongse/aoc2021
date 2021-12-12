//! Day 11: Dumbo Octopus, Advent of Code 2021
//! <https://adventofcode.com/2021/day/11>
use std::collections::HashMap;
use std::io::BufRead;

use anyhow::Context;

use aoc2021::argparser;
use aoc2021::try_collect::TryCollectArray;

fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_reader().expect("cannot open file");
    let graph = parse_input(input_reader).expect("cannot parse input");

    // Part 1: exhaustive search for all paths from start to end
    let p1_answer = graph.exhaustive_paths("start", "end");
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer: usize = 0;
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
        self.adjlists.entry(u).or_insert_with(|| Vec::new()).push(v);
    }

    /// Exhaustive path searching from start to end.
    fn exhaustive_paths<T: AsRef<str>>(&self, start: T, end: T) -> usize {
        #[derive(Debug, Eq, PartialEq)]
        enum Event<'a> {
            PreStack(&'a str),
            InStack(&'a str),
        }
        let start = start.as_ref();
        let end = end.as_ref();
        let mut count = 0;

        let mut event_stack = Vec::from([Event::PreStack(start)]);
        let mut depth_stack = Vec::new();
        while let Some(event) = event_stack.pop() {
            match event {
                Event::PreStack(curr) if curr == end => {
                    eprintln!("{:?}", depth_stack);
                    count += 1;
                }
                Event::PreStack(curr) => {
                    event_stack.push(Event::InStack(curr));
                    depth_stack.push(curr);
                    for next in self.adjlists[curr].iter() {
                        if next.chars().all(char::is_uppercase) || !depth_stack.contains(&&**next) {
                            event_stack.push(Event::PreStack(next))
                        }
                    }
                }
                Event::InStack(_curr) => {
                    depth_stack.pop();
                }
            }
        }
        count
    }
}
