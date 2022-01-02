//! Implements a simplified version of program argument parser.
use std::io::{stdin, Read};
use std::path::{Path, PathBuf};

use clap::Parser;

/// Command line argument parser for aoc2021 solver programs
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Path to an input file (or specify '-' for standard input)
    #[clap(parse(from_os_str))]
    pub input_file: Option<PathBuf>,
}

impl Cli {
    /// Obtains a raw reader for the input file.
    /// If the input file is empty of '-', then standard input will be used instead.
    pub fn input_reader(&self) -> anyhow::Result<Box<dyn Read>> {
        let input_file = match self.input_file.as_deref() {
            Some(s) if s == Path::new("-") => None,
            v => v,
        };
        let input_reader: Box<dyn Read> = match input_file {
            None => Box::new(stdin()),
            Some(path) => Box::new(std::fs::File::open(path)?),
        };
        Ok(input_reader)
    }
}
