//! Provides Snailfish number parser for Advent of Code Day 18.
use lalrpop_util::lalrpop_mod;

pub use snailfish_syntax::ExprParser;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    snailfish_syntax
);

/// Node in a snailfish number
#[derive(Debug, Clone)]
pub enum Node {
    Branch(Box<Node>, Box<Node>),
    Leaf(i64),
}
